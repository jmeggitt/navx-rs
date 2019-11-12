use std::io::{self, ErrorKind};
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{spawn, JoinHandle};
use parking_lot::{RwLock, Mutex, MutexGuard};

use crate::protocol::Request;


pub trait Watch {
    type Out;

    fn watch<T>(self) -> Self::Out;
}

pub struct Watcher<T, S> {
    inner: Mutex<S>,
    cache: RwLock<io::Result<T>>,
    stop_indicator: AtomicBool,
}

unsafe impl<T, S> Sync for Watcher<T, S> {}

impl<T, S> Watcher<T, S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner: Mutex::new(inner),
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

            let value_read = self.inner.lock().read();

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

impl<T: Copy, S> Watched<T, S> {
    pub fn get(&self) -> io::Result<T> {
        match &*self.cache.read() {
            Ok(x) => Ok(*x),
            Err(e) => Err(clone_err(e)),
        }
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
