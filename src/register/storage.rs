// Copyright 2018 navx-rs Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! All addressable areas of interest in the navX register.

//use crate::register::packet::Packet;
use crate::protocol::{FromBuffer, FromBufferFallible};
use crate::register::packet::RegisterPacket;
use crate::serde::{
    read_radians, read_u16, CalibrationStatus, Capability, OperationStatus, SelfTestStatus,
    SensorStatus,
};

pub trait Addressable: FromBufferFallible + Send {
    const ADDRESS: u8;
    const LEN: usize;

    fn request() -> RegisterPacket {
        RegisterPacket::new(Self::ADDRESS, Self::LEN as u8)
    }
}

pub struct Identity {
    pub identity: u8,
    pub board_revision: u8,
    pub firmware_major_version: u8,
    pub firmware_minor_version: u8,
}

impl Addressable for Identity {
    const ADDRESS: u8 = 0x00;
    const LEN: usize = 4;
}

impl FromBuffer for Identity {
    fn read(buf: &[u8]) -> Self {
        Self {
            identity: buf[0],
            board_revision: buf[1],
            firmware_major_version: buf[2],
            firmware_minor_version: buf[3],
        }
    }
}

pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Addressable for Quaternion {
    const ADDRESS: u8 = 0x2A;
    const LEN: usize = 4;
}

impl FromBuffer for Quaternion {
    fn read(buf: &[u8]) -> Self {
        Self {
            w: read_radians(&buf[0..2]),
            x: read_radians(&buf[2..4]),
            y: read_radians(&buf[4..6]),
            z: read_radians(&buf[6..8]),
        }
    }
}

/// Configuration and limits of board. The units for the following fields are update_rate (Hz),
/// accel_fsr (Degrees/sec), and gyro_fsr (G).
pub struct Config {
    pub update_rate: u8,
    pub accel_fsr: u8,
    pub gyro_fsr: u16,
}

impl Addressable for Config {
    const ADDRESS: u8 = 0x04;
    const LEN: usize = 4;
}

impl FromBuffer for Config {
    fn read(buf: &[u8]) -> Self {
        Self {
            update_rate: buf[0],
            accel_fsr: buf[1],
            gyro_fsr: read_u16(&buf[2..4]),
        }
    }
}

pub struct Status {
    pub operation_status: OperationStatus,
    pub calibration_status: CalibrationStatus,
    pub self_test_status: SelfTestStatus,
    pub capabilities: Capability,
    pub sensor_status: SensorStatus,
}

impl Addressable for Status {
    const ADDRESS: u8 = 0x08;
    const LEN: usize = 9;
}

impl FromBufferFallible for Status {
    // FIXME: These are not the correct register locations to read
    fn try_read(buf: &[u8]) -> Option<Self> {
        Some(Self {
            operation_status: OperationStatus::try_read(&buf[0..1])?,
            calibration_status: CalibrationStatus::read(&buf[1..2]),
            self_test_status: SelfTestStatus::read(&buf[2..3]),
            capabilities: Capability::read(&buf[3..4]),
            sensor_status: SensorStatus::read(&buf[4..9]),
        })
    }
}
