// Copyright 2018 navx-rs Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{FromBuffer, FromBufferFallible};
use byteorder::{ByteOrder, LittleEndian};
use std::f32::consts::PI;
use std::mem::transmute_copy;
use std::str::FromStr;

/*******************************************************************/
/*                      Register Definitions                       */
/*******************************************************************/
/* NOTE:  All multi-byte registers are in little-endian format.    */
/*        All registers with 'signed' data are twos-complement.    */
/*        Data Type Summary:                                       */
/*        unsigned byte:           0   - 255    (8 bits)           */
/*        unsigned short:          0   - 65535  (16 bits)          */
/*        signed short:        -32768  - 32767  (16 bits)          */
/*        signed hundredeths:  -327.68 - 327.67 (16 bits)		   */
/*        unsigned hundredths:    0.0  - 655.35 (16 bits)          */
/*        signed thousandths:  -32.768 - 32.767 (16 bits)          */
/*        signed short ratio: -1/16384 - 1/16384 (16 bits)         */
/*        16:16:           -32768.9999 - 32767.9999 (32 bits)      */
/*        unsigned long:             0 - 4294967295 (32 bits)      */
/*******************************************************************/

macro_rules! impl_read {
    ($buf:ident -> $ty:ty = $ret:expr) => {
        impl FromBuffer for $ty {
            fn read($buf: &[u8]) -> Self {
                $ret
            }
        }
    };
}

// TODO: Remove?
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

/// ASCII float (idk why they thought this was a good idea)
/// Format: [- ][0-9][0-9][0-9].[0-9][0-9]
pub fn read_float(buf: &[u8]) -> Option<f32> {
    f32::from_str(std::str::from_utf8(buf).ok()?).ok()
}

/// ASCII byte as hex (idk why they thought this was a good idea)
/// Format: [0-9A-F][0-9A-F]
pub fn read_byte(buf: &[u8]) -> Option<u8> {
    u8::from_str_radix(std::str::from_utf8(buf).ok()?, 16).ok()
}

/// ASCII byte as hex (idk why they thought this was a good idea)
/// Format: [0-9A-F][0-9A-F][0-9A-F][0-9A-F]
pub fn read_int(buf: &[u8]) -> Option<u16> {
    u16::from_str_radix(std::str::from_utf8(buf).ok()?, 16).ok()
}

bitflags! {
    pub struct SensorStatus: u8 {
        const MOVING              = 0b0000_0001;
        const YAW_STABLE          = 0b0000_0010;
        const MAG_DISTURBANCE     = 0b0000_0100;
        const ALTITUDE_VALID      = 0b0000_1000;
        const SEALEVEL_PRESS_SET  = 0b0001_0000;
        const FUSED_HEADING_VALID = 0b0010_0000;
    }
}

bitflags! {
    pub struct CalibrationStatus: u8 {
        const IMU_INPROGRESS = 0b0000_0000;
        const IMU_ACCUMULATE = 0b0000_0001;
        const IMU_COMPLETE   = 0b0000_0010;
        const MAG_COMPLETE   = 0b0000_0100;
        const BARO_COMPLETE  = 0b0000_1000;
    }
}

bitflags! {
    pub struct SelfTestStatus: u8 {
        const GYRO_PASSED  = 0b0000_0001;
        const ACCEL_PASSED = 0b0000_0010;
        const MAG_PASSED   = 0b0000_0100;
        const BARO_PASSED  = 0b0000_1000;
        const COMPLETE     = 0b1000_0000;
    }
}

// Keep in mind that there are two flags for velocity and displacement. I suspect they are
// equivalent to each other but from different models.
bitflags! {
    pub struct Capability: u8 {
        const OMNIMOUNT             = 0b0000_0100;
        const OMNIMOUNT_CONFIG_MASK = 0b0011_1000;
        const VEL_AND_DISP          = 0b0100_0000;
        const VEL_AND_DISP2         = 0b1000_0000;
    }
}

bitflags! {
    pub struct ControlReset: u8 {
        // Velocity
        const VEL_X  = 0b0000_0001;
        const VEL_Y  = 0b0000_0010;
        const VEL_Z  = 0b0000_0100;

        // Displacement
        const DISP_X = 0b0000_1000;
        const DISP_Y = 0b0001_0000;
        const DISP_Z = 0b0010_0000;

        // Yaw rotation
        const YAW    = 0b1000_0000;

        // Flags for convenience
        const VEL    = Self::VEL_X.bits | Self::VEL_Y.bits | Self::VEL_Z.bits;
        const DISP   = Self::DISP_X.bits | Self::DISP_Y.bits | Self::DISP_Z.bits;
        const POSE   = Self::VEL.bits | Self::DISP.bits;
        const ALL    = Self::POSE.bits | Self::YAW.bits;
    }
}

impl_read!(buf -> SensorStatus = Self::from_bits_truncate(buf[0]));
impl_read!(buf -> CalibrationStatus = Self::from_bits_truncate(buf[0]));
impl_read!(buf -> SelfTestStatus = Self::from_bits_truncate(buf[0]));
impl_read!(buf -> Capability = Self::from_bits_truncate(buf[0]));
impl_read!(buf -> ControlReset = Self::from_bits_truncate(buf[0]));

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum OperationStatus {
    Initializing = 0,
    RunningSelfTest = 1,
    Error = 2,
    Calibrating = 3,
    Normal = 4,
}

impl FromBufferFallible for OperationStatus {
    fn try_read(buf: &[u8]) -> Option<Self> {
        match buf[0] {
            x if x <= 4 => unsafe { Some(transmute_copy(&x)) },
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum OmniMountConfig {
    Default = 0,
    XUp = 1,
    XDown = 2,
    YUp = 3,
    YDown = 4,
    ZUp = 5,
    ZDown = 6,
}

impl FromBufferFallible for OmniMountConfig {
    fn try_read(buf: &[u8]) -> Option<Self> {
        match buf[0] {
            x if x <= 6 => unsafe { Some(transmute_copy(&x)) },
            _ => None,
        }
    }
}

/// The stream type used in the stream configuration command
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum StreamType {
    Directional = b'y',
    RawData = b'g',
    Position = b'p',
}

impl FromBufferFallible for StreamType {
    fn try_read(buf: &[u8]) -> Option<Self> {
        match buf[0] {
            b'y' => Some(Self::Directional),
            b'g' => Some(Self::RawData),
            b'p' => Some(Self::Position),
            _ => None,
        }
    }
}

/// A Helper struct representing a vector in 3D space.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
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
