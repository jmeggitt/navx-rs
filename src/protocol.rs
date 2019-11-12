//! Handle switching between register and serial protocol
use std::io::{self, Read, Write};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

use parking_lot::RwLock;

//use crate::register::packet::Packet;

pub trait FromBufferFallible: Sized {
    fn try_read(buf: &[u8]) -> Option<Self>;
}

pub trait FromBuffer: Sized {
    fn read(buf: &[u8]) -> Self;
}

impl<T: FromBuffer> FromBufferFallible for T {
    fn try_read(buf: &[u8]) -> Option<Self> {
        Some(<Self as FromBuffer>::read(buf))
    }
}

// TODO: Do I need this trait? I could probably get rid of it with very little effort.
pub trait Packet {
    fn len(&self) -> usize;
    fn pack<'a>(self) -> &'a [u8];
    fn pack_write<'a>(self) -> &'a [u8];
}

/// The handle you hold to get information from the navX
/// Maybe deref into its inner Register/Serial system?
pub struct NavX<IO: BoardIO> {
    inner: IO,
    spec: BoardSpec,
}

impl<IO: BoardIO> Deref for NavX<IO> {
    type Target = IO;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<IO: BoardIO> DerefMut for NavX<IO> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Storage for the different status flags and capabilities of this board
pub struct BoardSpec {}

pub trait BoardIO {
    type PacketType: Packet;

    fn write(&mut self, packet: Self::PacketType) -> io::Result<()>;

    // Basic shared traits between registerIO and serialIO
}

pub struct SerialIO<T> {
    inner: T,
}

//impl<T> BoardIO for SerialIO<T> {
//
//}

pub trait Request<T> {
    /// Request to read a value. This operation is blocking!
    fn read(&mut self) -> io::Result<T>;
}
