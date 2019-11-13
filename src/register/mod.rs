//! A module for dealing with the register protocol of the navX.
use std::io::{self, ErrorKind, Read, Write};
use std::ops::{Deref, DerefMut};

use crate::register::storage::Addressable;
use crate::watch::{Watch, Watched};
use crate::{get_crc, Packet, Request};

pub mod packet;
pub mod storage;

pub struct RegisterIO<T> {
    inner: T,
    buffer: [u8; 0x100],
}

impl<T: Write> RegisterIO<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            buffer: [0; 0x100],
        }
    }

    /// Write a packet
    fn write<P: Packet>(&mut self, packet: P) -> io::Result<()> {
        let expected_len = packet.len();

        match self.inner.write(packet.pack_write()) {
            Ok(x) if x == expected_len => Ok(()),
            Ok(_) => Err(ErrorKind::Interrupted.into()),
            Err(e) => Err(e),
        }
    }
}

// TODO: Will this approach to reading also work for serial inputs
impl<T: Read + Write, V: Addressable> Request<V> for RegisterIO<T> {
    fn read(&mut self) -> io::Result<V> {
        self.write(V::request())?;

        // Read bytes
        if self.inner.read(&mut self.buffer[..=V::LEN])? != (V::LEN + 1) as usize {
            return Err(ErrorKind::Interrupted.into());
        }

        // Checksum
        if get_crc(&self.buffer[..V::LEN], V::LEN) != self.buffer[V::LEN] {
            return Err(ErrorKind::InvalidData.into());
        }

        // Parse bytes to type
        match V::try_read(&self.buffer[..V::LEN]) {
            Some(x) => Ok(x),
            None => Err(ErrorKind::InvalidData.into()),
        }
    }
}

impl<T: 'static + Read + Write + Send, V: 'static + Addressable> Watch<V> for RegisterIO<T> {
    type Provider = Self;

    // No additional setup required
    fn watch(self) -> Watched<V, Self::Provider> {
        Watched::new(self)
    }
}

impl<T> Deref for RegisterIO<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for RegisterIO<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

//impl<T> BoardIO for RegisterIO<T> {
//
//}
