// Copyright 2018 navx-rs Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! All addressable areas of interest for the navX register map

use crate::spia::Packet;
use crate::types::{
    CalibrationStatus, Capability, IRadian, OperationStatus, RegType, SelfTestStatus, SensorStatus,
    U16,
};

pub trait Addressable {
    const ADDRESS: u8;
    const LEN: u8;

    fn read(buffer: &[u8]) -> Self;

    fn request() -> Packet {
        Packet::read(Self::ADDRESS, Self::LEN)
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
    const LEN: u8 = 4;

    fn read(buffer: &[u8]) -> Self {
        Self {
            identity: buffer[0],
            board_revision: buffer[1],
            firmware_major_version: buffer[2],
            firmware_minor_version: buffer[3],
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
    const LEN: u8 = 4;

    fn read(buffer: &[u8]) -> Self {
        Self {
            w: IRadian::read(&buffer[..]),
            x: IRadian::read(&buffer[2..]),
            y: IRadian::read(&buffer[4..]),
            z: IRadian::read(&buffer[6..]),
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
    const LEN: u8 = 4;

    fn read(buffer: &[u8]) -> Self {
        Self {
            update_rate: buffer[0],
            accel_fsr: buffer[1],
            gyro_fsr: U16::read(&buffer[2..4]),
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
    const LEN: u8 = 9;

    // FIXME: These are not the correct register locations to read
    fn read(buffer: &[u8]) -> Self {
        Self {
            operation_status: OperationStatus::read(&buffer[..]),
            calibration_status: CalibrationStatus::read(&buffer[1..]),
            self_test_status: SelfTestStatus::read(&buffer[2..]),
            capabilities: Capability::read(&buffer[3..]),
            sensor_status: SensorStatus::read(&buffer[4..]),
        }
    }
}
