use std::error::Error;
use std::io::{self, ErrorKind};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

// Parking lot is faster, but std allows values to be taken out
use crate::protocol::Request;
use parking_lot::{Mutex, RwLock};

pub trait Watch {
    type Out;

    /// Setup a Watcher using self as the data provider.
    fn watch<T>(self) -> Self::Out;
}

pub struct Watcher<T, S> {
    inner: Mutex<Option<S>>,
    cache: RwLock<io::Result<T>>,
    stop_indicator: AtomicBool,
}

/// Watcher is guaranteed to be thread safe because all of its contents are thread safe. However, it
/// will require an Arc to send to another thread.
unsafe impl<T, S> Sync for Watcher<T, S> {}

impl<T, S> Watcher<T, S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner: Mutex::new(Some(inner)),
            cache: RwLock::new(Err(io::Error::new(
                ErrorKind::Interrupted,
                "Value has not been read yet",
            ))),
            stop_indicator: AtomicBool::new(false),
        }
    }
}

impl<T: 'static + Send, S: 'static + Request<T> + Send> Watcher<T, S> {
    pub fn start(self: Arc<Self>) -> JoinHandle<()> {
        spawn(move || loop {
            if self.stop_indicator.load(Ordering::SeqCst) {
                return;
            }

            let value_read = self.inner.lock().as_mut().unwrap().read();

            match value_read {
                Ok(v) => *self.cache.write() = Ok(v),
                Err(ref e) if e.kind() == ErrorKind::Interrupted => (),
                x => {
                    *self.cache.write() = x;
                    self.stop_indicator.store(true, Ordering::SeqCst);
                    return;
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
    /// Stops watching the value and makes the watcher thread join this thread. If an IO error
    /// occurred while watching, that error will be returned instead.
    pub fn stop(self) -> io::Result<S> {
        self.stop_indicator.store(true, Ordering::SeqCst);

        if self.join_handle.join().is_err() {
            return Err(io::Error::new(ErrorKind::BrokenPipe, ""));
        }

        match &*self.inner.cache.read() {
            Ok(_) => Ok(self.inner.inner.lock().take().unwrap()),
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                Ok(self.inner.inner.lock().take().unwrap())
            }
            Err(ref e) => Err(clone_err(e)),
        }
    }

    /// Checks if the inner thread is running
    pub fn is_stopped(&self) -> bool {
        self.inner.stop_indicator.load(Ordering::SeqCst)
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
        return io::Error::from_raw_os_error(os_err);
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
    /// Get the latest recorded value. If an IO error occurs, the watcher will stop updating and
    /// store the error. The exception to this is interrupted errors since they are often not fatal.
    /// Keep in mind that if this method is called before the first value can be read, an
    /// interrupted error will be returned. This error is not fatal and will be replaced with a
    /// valid answer once one is found.
    pub fn get(&self) -> io::Result<T> {
        clone_io_result(&*self.cache.read())
    }

    /// Checks if this watcher has collected a valid value yet.
    pub fn is_ready(&self) -> bool {
        match self.get() {
            Err(ref e) if e.kind() == ErrorKind::Interrupted => false,
            // Return true even if an error is found to prevent programs from hanging
            _ => true,
        }
    }
}
