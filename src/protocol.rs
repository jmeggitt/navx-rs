//! Handle switching between register and serial protocol
use std::io;
use std::ops::{Deref, DerefMut};
use crate::register::RegisterIO;
use wpilib::spi::Spi;

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
pub struct NavX<T> {
    inner: T,
    spec: BoardSpec,
}

impl<T> NavX<T> {
    /// Retrieve board specs
    pub fn init(io: T) -> Self {
        unimplemented!()
    }

    pub fn new_spi(mut port: Spi) -> Self {
        const DEFAULT_BITRATE: u32 = 500000; // TODO: Find correct rate

        port.set_clock_rate(f64::from(DEFAULT_BITRATE));
        port.set_msb_first();
        port.set_sample_data_on_trailing_edge();
        port.set_clock_active_low();

        Self::init(RegisterIO::new(port))
    }
}

impl<T> Deref for NavX<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for NavX<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Storage for the different status flags and capabilities of this board
pub struct BoardSpec {}

// TODO: Replace with alternate trait that works better
//pub trait BoardIO {
//    type PacketType: Packet;
//
//    fn write(&mut self, packet: Self::PacketType) -> io::Result<()>;
//
//    // Basic shared traits between registerIO and serialIO
//}

// TODO: Move to serial module
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


const CRC7_POLY: u8 = 0x91;

/// I was unable to get the same results with any crc library. The navx chip may not use a standard
/// crc7 implementation.
pub fn get_crc(message: &[u8], length: usize) -> u8 {
    let mut crc = 0;

    for i in 0..length as u8 {
        crc ^= message[i as usize];
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc ^= CRC7_POLY;
            }
            crc >>= 1;
        }
    }
    crc
}

