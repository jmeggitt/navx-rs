// Copyright 2018 navx-rs Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use std::f32::consts::PI;
use std::mem::transmute;
use std::ops::Deref;

/// A type used in the NavX-MXP registers
pub trait RegType {
    /// The size of this type in bytes of the value in the register
    const SIZE: usize;
    /// The equivalent rust type this can be parsed to
    type Output;

    /// Parse the rust type by reading from the given buffer. This assumes that the data is at the
    /// beginning of the slice and that there are enough bytes within the slice.
    fn read(buffer: &[u8]) -> Self::Output;
}

// TODO: Consider replacing RegType impls with macro
macro_rules! impl_reg {
    ($T:ty [$size:expr; $O:ty] $buf:ident -> $get:expr) => {
        impl RegType for $T {
            const SIZE: usize = $size;
            type Output = $O;

            fn read($buf: &[u8]) -> $O {
                $get
            }
        }
    };
}

// The base types available to the navX
pub struct U8;
pub struct U16;
pub struct I16;
pub struct IHundredth;
pub struct UHundredth;
pub struct IThousandth;
pub struct IRadian;
pub struct Q16;
pub struct U32;

// Implement base types
impl_reg!(U8 [1; u8] buf -> buf[0]);
impl_reg!(U16 [2; u16] buf -> LittleEndian::read_u16(buf));
impl_reg!(I16 [2; i16] buf -> LittleEndian::read_i16(buf));
impl_reg!(IHundredth [2; f32] buf -> LittleEndian::read_i16(buf) as f32 / 100.0);
impl_reg!(UHundredth [2; f32] buf -> LittleEndian::read_u16(buf) as f32 / 100.0);
impl_reg!(IThousandth [2; f32] buf -> LittleEndian::read_i16(buf) as f32 / 1000.0);
impl_reg!(IRadian [2; f32] buf -> LittleEndian::read_i16(buf) as f32 * PI / 16384.0);
impl_reg!(Q16 [4; f64] buf -> LittleEndian::read_u32(buf) as f64 / 66536.0);
impl_reg!(U32 [4; u32] buf -> LittleEndian::read_u32(buf));

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

// Implement RegType for all of the bitflags types
impl_reg!(SensorStatus [1; Self] buf -> Self::from_bits_truncate(buf[0]));
impl_reg!(CalibrationStatus [1; Self] buf -> Self::from_bits_truncate(buf[0]));
impl_reg!(SelfTestStatus [1; Self] buf -> Self::from_bits_truncate(buf[0]));
impl_reg!(Capability [1; Self] buf -> Self::from_bits_truncate(buf[0]));
impl_reg!(ControlReset [1; Self] buf -> Self::from_bits_truncate(buf[0]));

#[repr(u8)]
pub enum OperationStatus {
    Initializing = 0,
    RunningSelfTest = 1,
    Error = 2,
    Calibrating = 3,
    Normal = 4,
}

impl RegType for OperationStatus {
    const SIZE: usize = 1;
    type Output = Self;

    fn read(buffer: &[u8]) -> Self::Output {
        // Clamp to ensure this will always produce a valid status
        unsafe { transmute(buffer[0].min(4)) }
    }
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

impl RegType for OmniMountConfig {
    const SIZE: usize = 1;
    type Output = Self;

    fn read(buffer: &[u8]) -> Self::Output {
        // Clamp to ensure this will always produce a valid status
        unsafe { transmute(buffer[0].min(6)) }
    }
}

/// A Helper struct representing a vector in 3D space.
pub struct Vector<T: RegType> {
    pub x: T::Output,
    pub y: T::Output,
    pub z: T::Output,
}

impl<T: RegType> RegType for Vector<T> {
    const SIZE: usize = 3 * T::SIZE;
    type Output = Self;

    fn read(buffer: &[u8]) -> Self::Output {
        Self {
            x: T::read(&buffer[..T::SIZE]),
            y: T::read(&buffer[T::SIZE..2 * T::SIZE]),
            z: T::read(&buffer[2 * T::SIZE..]),
        }
    }
}
