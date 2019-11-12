use std::io::{self, ErrorKind};
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{spawn, JoinHandle};

// Parking lot is faster, but std allows values to be taken out
use parking_lot::{RwLock, Mutex};
use crate::protocol::Request;

pub trait Watch {
    type Out;

    fn watch<T>(self) -> Self::Out;
}

pub struct Watcher<T, S> {
    inner: Mutex<Option<S>>,
    cache: RwLock<io::Result<T>>,
    stop_indicator: AtomicBool,
}

unsafe impl<T, S> Sync for Watcher<T, S> {}

impl<T, S> Watcher<T, S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner: Mutex::new(Some(inner)),
            cache: RwLock::new(Err(io::Error::new(ErrorKind::Interrupted, "Value has not been read yet"))),
            stop_indicator: AtomicBool::new(false),
        }
    }
}

impl<T: 'static + Send, S: 'static + Request<T> + Send> Watcher<T, S> {
    pub fn start(self: Arc<Self>) -> JoinHandle<()> {
        spawn(move || loop {
            if self.stop_indicator.load(Ordering::SeqCst) {
                return
            }

            let value_read = self.inner.lock().as_mut().unwrap().read();

            match value_read {
                Ok(v) => *self.cache.write() = Ok(v),
                Err(ref e) if e.kind() == ErrorKind::Interrupted => (),
                x => {
                    *self.cache.write() = x;
                    self.stop_indicator.store(true, Ordering::SeqCst);
                    return
                }
            }
        })
    }
}


pub struct Watched<T, S> {
    inner: Arc<Watcher<T, S>>,
    join_handle: JoinHandle<()>,
}

impl<T: 'static + Send, S: 'static + Request<T> + Send> Watched<T, S> {
    pub fn new(inner: S) -> Self {
        let watcher = Arc::new(Watcher::new(inner));

        Self {
            inner: watcher.clone(),
            join_handle: watcher.start(),
        }
    }
}

impl<T, S> Watched<T, S> {
    /// Stops watching the value and makes the watcher thread join this thread. If an IO error occurred while watching, that error will be returned instead.
    pub fn stop(self) -> io::Result<S> {
        self.stop_indicator.store(true, Ordering::SeqCst);

        if self.join_handle.join().is_err() {
            return Err(io::Error::new(ErrorKind::BrokenPipe, ""))
        }

        match &*self.inner.cache.read() {
            Ok(_) => Ok(self.inner.inner.lock().take().unwrap()),
            Err(ref e) if e.kind() == ErrorKind::Interrupted => Ok(self.inner.inner.lock().take().unwrap()),
            Err(ref e) => Err(clone_err(e)),
        }
    }
}

impl<T, S> Deref for Watched<T, S> {
    type Target = Arc<Watcher<T, S>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, S> DerefMut for Watched<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


/// io::Error does not implement Clone (https://github.com/rust-lang/rust/issues/24135) so this work
/// around is used instead.
fn clone_err(err: &io::Error) -> io::Error {
    // Preserve os errors
    if let Some(os_err) = err.raw_os_error() {
        return io::Error::from_raw_os_error(os_err)
    }

    // Custom errors can't be retrieved, so make sure to preserve the debug print
    io::Error::new(err.kind(), err.description().to_owned())
}

fn clone_io_result<T: Copy>(res: &io::Result<T>) -> io::Result<T> {
    match res {
        Ok(x) => Ok(*x),
        Err(e) => Err(clone_err(e)),
    }
}

impl<T: Copy, S> Watched<T, S> {
    pub fn get(&self) -> io::Result<T> {
        clone_io_result(&*self.cache.read())
    }
}

