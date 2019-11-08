use std::io::{self, Read};

use memchr::memchr;
use std::convert::TryFrom;

const PACKET_INDICATOR: u8 = b'!';

pub struct PacketReader<T: Read> {
    inner: T,
    buf: [u8; 256],
    start: usize,
    pos: usize,
}

impl<T: Read> PacketReader<T> {
    pub fn new(reader: T) -> Self {
        Self {
            inner: reader,
            buf: [0; 256],
            pos: 0,
            start: 0,
        }
    }

    fn try_read(&mut self) -> io::Result<bool> {
        let len = self.inner.read(&mut self.buf[self.pos..])?;

        if len == 0 {
            return Ok(false);
        }

        match memchr(PACKET_INDICATOR, &self.buf[self.pos..self.pos + len]) {
            Some(x) => {
                self.start += x;
            }
            None => (),
        };

        Ok(false)
    }
}

#[repr(C, packed)]
struct Packet<T> {
    indicator: u8,
    msg_id: u8,
    body: T,
    termination: PacketTermination,
}

impl<T> TryFrom<&[u8]> for Packet<T> {
    type Error = PacketParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value[0] != PACKET_INDICATOR {
            return Err(PacketParseError::Malformed);
        }

        match value[1] {
            b'y' => (), // Heading update
            b'g' => (), // Raw data update
            b'S' => (), // Stream configuration command
            b's' => (), // Stream configuration response
            b'#' => (), // Binary packet indicator
            _ => return Err(PacketParseError::Malformed),
        };

        unimplemented!()
    }
}

enum PacketParseError {
    Unfinished,
    Malformed,
}

#[repr(C, packed)]
struct BinaryPacket<T> {
    len: u8,
    msg_id: u8,
    body: T,
}

#[repr(C, packed)]
struct PacketTermination {
    // crc7 checksum
    checksum: [u8; 2],
    // characters \r\n (unchecked, but included for correctness)
    _break: u16,
}

impl PacketTermination {
    fn checksum(&self) -> Option<u8> {
        u8::from_str_radix(std::str::from_utf8(&self.checksum[..]).ok()?, 16).ok()
    }
}
