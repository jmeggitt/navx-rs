use crate::protocol::{FromBuffer, FromBufferFallible};
use crate::serde::*;
use std::convert::TryInto;

// TODO: Find a shared place to put Quaternions
use crate::register::storage::{Addressable, Quaternion};

//pub trait ReadPacket: Sized {
//    /// Returns an option because even though packets need to pass a checksum, the parsing still can
//    /// fail because of non-binary types.
//    fn read(buf: &[u8]) -> Option<Self>;
//}

/// A directional yaw/pitch/roll/heading update. I decided against using a vector for the
/// yaw/pitch/roll to preserve the clear naming.
pub struct DirectionalUpdate {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub compass_heading: f32,
}

impl FromBufferFallible for DirectionalUpdate {
    fn try_read(buf: &[u8]) -> Option<Self> {
        Some(Self {
            yaw: read_float(&buf[0..7])?,
            pitch: read_float(&buf[7..14])?,
            roll: read_float(&buf[14..21])?,
            compass_heading: read_float(&buf[21..28])?,
        })
    }
}

// TODO: Build in way to make units more readable
/// A raw data update from the NavX. The units are as follows:
/// gyro: deg/sec * gyro full scale range
/// acceleration: G * accelerometer full scale range
/// magnetometer: uTesla * 0.15
/// temperature: celsius
pub struct RawDataUpdate {
    pub gyro: Vector<i16>,
    pub acceleration: Vector<i16>,
    pub magnetometer: Vector<i16>,
    pub temperature: f32,
}

impl FromBufferFallible for RawDataUpdate {
    fn try_read(buf: &[u8]) -> Option<Self> {
        Some(Self {
            gyro: Vector::read(read_i16, &buf[0..12]),
            acceleration: Vector::read(read_i16, &buf[12..24]),
            magnetometer: Vector::read(read_i16, &buf[24..36]),
            temperature: read_float(&buf[36..43])?,
        })
    }
}

pub struct Status {
    pub operation: OperationStatus,
    pub sensor: SensorStatus,
    pub calibration: CalibrationStatus,
    pub self_test: SelfTestStatus,
}

impl FromBufferFallible for Status {
    fn try_read(buf: &[u8]) -> Option<Self> {
        Some(Self {
            operation: OperationStatus::try_read(&buf[0..1])?,
            sensor: SensorStatus::read(&buf[1..2]),
            calibration: CalibrationStatus::read(&buf[2..3]),
            self_test: SelfTestStatus::read(&buf[3..4]),
        })
    }
}

pub struct PositionUpdate {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub compass_heading: f32,
    pub altitude: f64,
    pub fused_heading: f32,
    pub linear_accel: Vector<f32>,
    pub linear_velocity: Vector<f64>,
    pub displacement: Vector<f64>,
    pub quaternion: Quaternion,
    pub mpu_temp: f32,
    pub status: Status,
}

impl FromBufferFallible for PositionUpdate {
    fn try_read(buf: &[u8]) -> Option<Self> {
        Some(Self {
            yaw: read_hundredth(&buf[0..2]),
            pitch: read_hundredth(&buf[2..4]),
            roll: read_hundredth(&buf[4..6]),
            compass_heading: read_uhundredth(&buf[6..8]),
            altitude: read_q1616(&buf[8..12]),
            fused_heading: read_uhundredth(&buf[12..14]),
            linear_accel: Vector::read(read_thousandth, &buf[14..20]),
            linear_velocity: Vector::read(read_q1616, &buf[20..32]),
            displacement: Vector::read(read_q1616, &buf[32..44]),
            quaternion: Quaternion::read(&buf[44..52]),
            mpu_temp: read_hundredth(&buf[52..54]),
            status: Status::try_read(&buf[54..58])?,
        })
    }
}

/// Note: FSR stands for Full Scale Range
pub struct StreamConfigurationResponse {
    pub stream_type: StreamType,
    pub gyro_fsr: u16,
    pub accel_fsr: u16,
    pub update_rate: u16,
    pub calibrated_yaw_offset: f32,
    // This part of the packet is reserved for any changes in the protocol
    pub reserved: [u8; 16],
    // Calibration Status somewhat misrepresents flags, but there if a fair amount of overlap.
    // This value should only be used to check the IMU status.
    pub flags: CalibrationStatus,
}

impl FromBufferFallible for StreamConfigurationResponse {
    fn try_read(buf: &[u8]) -> Option<Self> {
        Some(Self {
            stream_type: StreamType::try_read(&buf[0..1])?,
            gyro_fsr: read_int(&buf[1..5])?,
            accel_fsr: read_int(&buf[5..9])?,
            update_rate: read_int(&buf[9..13])?,
            calibrated_yaw_offset: read_float(&buf[13..20])?,
            reserved: buf[20..36].try_into().unwrap(),
            flags: CalibrationStatus::read(&buf[36..37]),
        })
    }
}

/// A command to configure what information is being collected from the NavX. This should be hidden
/// inside this library and only be used as needed.
pub struct StreamConfigurationCommand {
    stream_type: StreamType,
    update_rate: u8,
}
