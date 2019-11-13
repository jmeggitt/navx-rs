use std::slice::from_raw_parts;

use crate::protocol::{Packet, get_crc};

/// A packet to be sent to the NavX. Every packet is exactly three bytes long and can be broken into
/// four sections. NavX provided register mappings:
/// https://pdocs.kauailabs.com/navx-mxp/advanced/register-protocol/
///
/// Format: [read/write flag] [register address] [body] [checksum]
///  - **read/write flag (1 bit)**: A single bit that indicates if the packet will be reading or
///     writing to the register address. True indicates a write and false indicates a read.
///  - **register address (7 bits)**: The location to either read or write to. If writing, then this
///     is the address of the byte that the packet body will be put in. If reading, this is the
///     address that will be read from. In pseudocode, `registers[address..address + body]`
///     describes the result of a read.
///  - **body (1 byte)**: When writing, the packet body is the value to write to the given address.
///     When reading, the body is the number of bytes to read starting from a given register.
#[repr(C, packed)]
pub struct RegisterPacket {
    register: u8,
    value: u8,
    checksum: u8,
}

impl RegisterPacket {
    pub fn new(reg: u8, len: u8) -> Self {
        Self {
            register: reg,
            value: len,
            checksum: 0,
        }
    }

    fn apply_checksum(&mut self) {
        unsafe {
            let ptr = &self as *const _ as *const u8;
            self.checksum = get_crc(from_raw_parts(ptr, 2), 2);
        }
    }
}

impl Packet for RegisterPacket {
    /// Register packets will always be 3 long
    fn len(&self) -> usize {
        3
    }

    fn pack<'a>(mut self) -> &'a [u8] {
        self.apply_checksum();

        // Honestly, this probably won't even save much time
        let ptr = &self as *const _ as *const u8;
        unsafe { from_raw_parts(ptr, 3) }
    }

    fn pack_write<'a>(mut self) -> &'a [u8] {
        // Set a flag within the register to mark this packet as a write command see docs on
        // RegisterPacket for more information.
        self.register |= 0x80;
        self.pack()
    }
}
