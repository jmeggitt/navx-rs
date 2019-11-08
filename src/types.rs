// Copyright 2018 navx-rs Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! TODO: This is overcomplicated. Maybe go back to functions or try implementing some serde stuff.

use byteorder::{ByteOrder, LittleEndian};
use std::f32::consts::PI;
use std::mem::transmute;

macro_rules! impl_read {
    ($buf:ident -> $ty:ty = $ret:expr) => {
        impl $ty {
            pub fn read($buf: &[u8]) -> Self {
                $ret
            }
        }
    };
}

// For completeness
pub fn read_u8(buf: &[u8]) -> u8 {
    buf[0]
}

pub fn read_u16(buf: &[u8]) -> u16 {
    LittleEndian::read_u16(buf)
}

pub fn read_i16(buf: &[u8]) -> i16 {
    LittleEndian::read_i16(buf)
}

pub fn read_hundredth(buf: &[u8]) -> f32 {
    f32::from(LittleEndian::read_i16(buf)) / 100.0
}

pub fn read_uhundredth(buf: &[u8]) -> f32 {
    f32::from(LittleEndian::read_u16(buf)) / 100.0
}

pub fn read_thousandth(buf: &[u8]) -> f32 {
    f32::from(LittleEndian::read_i16(buf)) / 1000.0
}

pub fn read_radians(buf: &[u8]) -> f32 {
    f32::from(LittleEndian::read_i16(buf)) * PI / 16384.0
}

pub fn read_q1616(buf: &[u8]) -> f64 {
    f64::from(LittleEndian::read_u32(buf)) / 66536.0
}

pub fn read_u32(buf: &[u8]) -> u32 {
    LittleEndian::read_u32(buf)
}

bitflags! {
    pub struct SensorStatus: u8 {
        const MOVING              = 0b00000001;
        const YAW_STABLE          = 0b00000010;
        const MAG_DISTURBANCE     = 0b00000100;
        const ALTITUDE_VALID      = 0b00001000;
        const SEALEVEL_PRESS_SET  = 0b00010000;
        const FUSED_HEADING_VALID = 0b00100000;
    }
}

bitflags! {
    pub struct CalibrationStatus: u8 {
        const IMU_INPROGRESS = 0b00000000;
        const IMU_ACCUMULATE = 0b00000001;
        const IMU_COMPLETE   = 0b00000010;
        const MAG_COMPLETE   = 0b00000100;
        const BARO_COMPLETE  = 0b00001000;
    }
}

bitflags! {
    pub struct SelfTestStatus: u8 {
        const GYRO_PASSED  = 0b00000001;
        const ACCEL_PASSED = 0b00000010;
        const MAG_PASSED   = 0b00000100;
        const BARO_PASSED  = 0b00001000;
        const COMPLETE     = 0b10000000;
    }
}

// Keep in mind that there are two flags for velocity and displacement. I suspect they are
// equivalent to each other but from different models.
bitflags! {
    pub struct Capability: u8 {
        const OMNIMOUNT             = 0b00000100;
        const OMNIMOUNT_CONFIG_MASK = 0b00111000;
        const VEL_AND_DISP          = 0b01000000;
        const VEL_AND_DISP2         = 0b10000000;
    }
}

bitflags! {
    pub struct ControlReset: u8 {
        // Velocity
        const VEL_X  = 0b00000001;
        const VEL_Y  = 0b00000010;
        const VEL_Z  = 0b00000100;

        // Displacement
        const DISP_X = 0b00001000;
        const DISP_Y = 0b00010000;
        const DISP_Z = 0b00100000;

        // Yaw rotation
        const YAW    = 0b10000000;

        // Flags for convenience
        const VEL    = Self::VEL_X.bits | Self::VEL_Y.bits | Self::VEL_Z.bits;
        const DISP   = Self::DISP_X.bits | Self::DISP_Y.bits | Self::DISP_Z.bits;
        const POSE   = Self::VEL.bits | Self::DISP.bits;
        const ALL    = Self::POSE.bits | Self::YAW.bits;
    }
}

#[repr(u8)]
pub enum OperationStatus {
    Initializing = 0,
    RunningSelfTest = 1,
    Error = 2,
    Calibrating = 3,
    Normal = 4,
}

#[repr(u8)]
pub enum OmniMountConfig {
    Default = 0,
    XUp = 1,
    XDown = 2,
    YUp = 3,
    YDown = 4,
    ZUp = 5,
    ZDown = 6,
}

impl_read!(buf -> SensorStatus = Self::from_bits_truncate(buf[0]));
impl_read!(buf -> CalibrationStatus = Self::from_bits_truncate(buf[0]));
impl_read!(buf -> SelfTestStatus = Self::from_bits_truncate(buf[0]));
impl_read!(buf -> Capability = Self::from_bits_truncate(buf[0]));
impl_read!(buf -> ControlReset = Self::from_bits_truncate(buf[0]));
impl_read!(buf -> OperationStatus = unsafe { transmute(buf[0].min(4)) });
impl_read!(buf -> OmniMountConfig = unsafe { transmute(buf[0].min(6)) });

/// A Helper struct representing a vector in 3D space.
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector<T> {
    pub fn read(read: fn(&[u8]) -> T, buf: &[u8]) -> Self {
        let segment = buf.len() / 3;
        Self {
            x: read(&buf[..segment]),
            y: read(&buf[segment..2 * segment]),
            z: read(&buf[2 * segment..]),
        }
    }
}
