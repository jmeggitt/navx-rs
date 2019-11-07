/// Message handling for SPI (Maybe scrap?)
/// TODO: Fix module name (name collision with wpilib::spi and changing this name was way faster than doing it right)
use crate::registers;
use std::io::{Read, Write};
use std::slice::from_raw_parts;

#[repr(packed)]
pub struct Packet {
    register: u8,
    value: u8,
    checksum: u8,
}

impl Packet {
    pub fn read(reg: u8, len: u8) -> Self {
        Self {
            register: reg,
            value: len,
            checksum: 0,
        }
    }

    pub fn write(reg: u8, val: u8) -> Self {
        unimplemented!()
    }

    fn apply_checksum(&mut self) {
        unsafe {
            let ptr = &self as *const _ as *const u8;
            self.checksum = registers::get_crc(from_raw_parts(ptr, 2), 2);
        }
    }

    pub fn pack<'a>(mut self) -> &'a [u8] {
        unsafe {
            let ptr = &self as *const _ as *const u8;
            self.checksum = registers::get_crc(from_raw_parts(ptr, 2), 2);
            from_raw_parts(ptr, 3)
        }
    }

    pub fn pack_write<'a>(mut self) -> &'a [u8] {
        // Set a flag within the register to mark this packet as a write command
        self.register |= 0x80;
        self.pack()
    }
}
