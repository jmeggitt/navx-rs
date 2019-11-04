// Copyright 2018 navx-rs Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*******************************************************************/
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

/**********************************************/
/* Device Identification Registers            */
/**********************************************/

pub const NAVX_REG_WHOAMI: usize = 0x00; /* IMU_MODEL_XXX */
pub const NAVX_REG_HW_REV: usize = 0x01;
pub const NAVX_REG_FW_VER_MAJOR: usize = 0x02;
pub const NAVX_REG_FW_VER_MINOR: usize = 0x03;

/**********************************************/
/* Status and Control Registers               */
/**********************************************/

/* Read-write */
pub const NAVX_REG_UPDATE_RATE_HZ: usize = 0x04; /* Range:  4 - 50 [unsigned byte] */
/* Read-only */
/* Accelerometer Full-Scale Range:  in units of G [unsigned byte] */
pub const NAVX_REG_ACCEL_FSR_G: usize = 0x05;
/* Gyro Full-Scale Range (Degrees/Sec):  Range:  250, 500, 1000 or 2000 [unsigned short] */
pub const NAVX_REG_GYRO_FSR_DPS_L: usize = 0x06; /* Lower 8-bits of Gyro Full-Scale Range */
pub const NAVX_REG_OP_STATUS: usize = 0x08; /* NAVX_OP_STATUS_XXX */
pub const NAVX_REG_CAL_STATUS: usize = 0x09; /* NAVX_CAL_STATUS_XXX */
pub const NAVX_REG_SELFTEST_STATUS: usize = 0x0A; /* NAVX_SELFTEST_STATUS_XXX */
pub const NAVX_REG_CAPABILITY_FLAGS_L: usize = 0x0B;

/**********************************************/
/* Processed Data Registers                   */
/**********************************************/

pub const NAVX_REG_SENSOR_STATUS_L: usize = 0x10; /* NAVX_SENSOR_STATUS_XXX */
pub const NAVX_REG_SENSOR_STATUS_H: usize = 0x11;
/* Timestamp:  [unsigned long] */
pub const NAVX_REG_TIMESTAMP_L_L: usize = 0x12;

/* Yaw, Pitch, Roll:  Range: -180.00 to 180.00 [signed hundredths] */
/* Compass Heading:   Range: 0.00 to 360.00 [unsigned hundredths] */
/* Altitude in Meters:  In units of meters [16:16] */

pub const NAVX_REG_YAW_L: usize = 0x16; /* Lower 8 bits of Yaw     */
pub const NAVX_REG_ROLL_L: usize = 0x18; /* Lower 8 bits of Roll    */
pub const NAVX_REG_PITCH_L: usize = 0x1A; /* Lower 8 bits of Pitch   */
pub const NAVX_REG_HEADING_L: usize = 0x1C; /* Lower 8 bits of Heading */
pub const NAVX_REG_FUSED_HEADING_L: usize = 0x1E; /* Upper 8 bits of Fused Heading */
pub const NAVX_REG_ALTITUDE_D_L: usize = 0x22;

/* World-frame Linear Acceleration: In units of +/- G * 1000 [signed thousandths] */

pub const NAVX_REG_LINEAR_ACC_X_L: usize = 0x24; /* Lower 8 bits of Linear Acceleration X */
pub const NAVX_REG_LINEAR_ACC_Y_L: usize = 0x26; /* Lower 8 bits of Linear Acceleration Y */
pub const NAVX_REG_LINEAR_ACC_Z_L: usize = 0x28; /* Lower 8 bits of Linear Acceleration Z */

/* Quaternion:  Range -1 to 1 [signed short ratio] */

pub const NAVX_REG_QUAT_W_L: usize = 0x2A; /* Lower 8 bits of Quaternion W */
pub const NAVX_REG_QUAT_X_L: usize = 0x2C; /* Lower 8 bits of Quaternion X */
pub const NAVX_REG_QUAT_Y_L: usize = 0x2E; /* Lower 8 bits of Quaternion Y */
pub const NAVX_REG_QUAT_Z_L: usize = 0x30; /* Lower 8 bits of Quaternion Z */

/**********************************************/
/* Raw Data Registers                         */
/**********************************************/

/* Sensor Die Temperature:  Range +/- 150, In units of Centigrade * 100 [signed hundredths float */

pub const NAVX_REG_MPU_TEMP_C_L: usize = 0x32; /* Lower 8 bits of Temperature */

/* Raw, Calibrated Angular Rotation, in device units.  Value in DPS = units / GYRO_FSR_DPS [signed short] */

pub const NAVX_REG_GYRO_X_L: usize = 0x34;
pub const NAVX_REG_GYRO_Y_L: usize = 0x36;
pub const NAVX_REG_GYRO_Z_L: usize = 0x38;

/* Raw, Calibrated, Acceleration Data, in device units.  Value in G = units / ACCEL_FSR_G [signed short] */

pub const NAVX_REG_ACC_X_L: usize = 0x3A;
pub const NAVX_REG_ACC_Y_L: usize = 0x3C;
pub const NAVX_REG_ACC_Z_L: usize = 0x3E;

/* Raw, Calibrated, Un-tilt corrected Magnetometer Data, in device units.  1 unit = 0.15 uTesla [signed short] */

pub const NAVX_REG_MAG_X_L: usize = 0x40;

/* Calibrated Pressure in millibars Valid Range:  10.00 Max:  1200.00 [16:16 float]  */

pub const NAVX_REG_PRESSURE_DL: usize = 0x48;

/**********************************************/
/* Calibration Registers                      */
/**********************************************/

/* Quaternion Offset:  Range: -1 to 1 [signed short ratio]  */

pub const NAVX_REG_QUAT_OFFSET_Z_H: u8 = 0x55; /* Upper 8 bits of Quaternion Z */

/**********************************************/
/* Integrated Data Registers                  */
/**********************************************/

/* Integration Control (Write-Only)           */
pub const NAVX_REG_INTEGRATION_CTL: u8 = 0x56;

/* Velocity:  Range -32768.9999 - 32767.9999 in units of Meters/Sec      */

pub const NAVX_REG_VEL_X_I_L: usize = 0x58;
pub const NAVX_REG_VEL_Y_I_L: usize = 0x5C;
pub const NAVX_REG_VEL_Z_I_L: usize = 0x60;

/* Displacement:  Range -32768.9999 - 32767.9999 in units of Meters      */

pub const NAVX_REG_DISP_X_I_L: usize = 0x64;
pub const NAVX_REG_DISP_Y_I_L: usize = 0x68;
pub const NAVX_REG_DISP_Z_I_L: usize = 0x6C;
pub const NAVX_REG_DISP_Z_D_H: usize = 0x6F;

pub const NAVX_REG_LAST: usize = NAVX_REG_DISP_Z_D_H;

/* NAVX_CAL_STATUS */

pub const NAVX_CAL_STATUS_MAG_CAL_COMPLETE: u8 = 0x04;

/* NAVX_SENSOR_STATUS */

pub const NAVX_SENSOR_STATUS_MOVING: u8 = 0x01;
pub const NAVX_SENSOR_STATUS_YAW_STABLE: u8 = 0x02;
pub const NAVX_SENSOR_STATUS_MAG_DISTURBANCE: u8 = 0x04;
pub const NAVX_SENSOR_STATUS_ALTITUDE_VALID: u8 = 0x08;

/* NAVX_REG_CAPABILITY_FLAGS (Aligned w/NAV6 Flags, see imu.rs) */

pub const NAVX_CAPABILITY_FLAG_OMNIMOUNT: i16 = 0x0004;
pub const NAVX_CAPABILITY_FLAG_VEL_AND_DISP: i16 = 0x0040;
pub const NAVX_CAPABILITY_FLAG_YAW_RESET: i16 = 0x0080;
pub const NAVX_CAPABILITY_FLAG_AHRSPOS_TS: i16 = 0x0100;

/* NAVX_INTEGRATION_CTL */

pub const NAVX_INTEGRATION_CTL_RESET_DISP_X: u8 = 0x08;
pub const NAVX_INTEGRATION_CTL_RESET_DISP_Y: u8 = 0x10;
pub const NAVX_INTEGRATION_CTL_RESET_DISP_Z: u8 = 0x20;

pub const NAVX_INTEGRATION_CTL_RESET_YAW: u8 = 0x80;

use byteorder::{ByteOrder, LittleEndian};

#[inline(always)]
pub fn dec_prot_u16(b: &[u8]) -> u16 {
    LittleEndian::read_u16(b)
}

#[inline(always)]
pub fn dec_prot_i16(b: &[u8]) -> i16 {
    LittleEndian::read_i16(b)
}

#[inline(always)]
pub fn dec_prot_u32(b: &[u8]) -> u32 {
    LittleEndian::read_u32(b)
}

#[inline(always)]
pub fn dec_prot_i32(b: &[u8]) -> i32 {
    LittleEndian::read_i32(b)
}

/* -327.68 to +327.68 */
#[inline(always)]
pub fn dec_prot_signed_hundreths_float(b: &[u8]) -> f32 {
    let mut signed_angle = f32::from(dec_prot_i16(b));
    signed_angle /= 100.;
    signed_angle
}

/* 0 to 655.35 */
#[inline(always)]
pub fn decodeProtocolUnsignedHundredthsFloat(b: &[u8]) -> f32 {
    let mut unsigned_float = f32::from(dec_prot_u16(b));
    unsigned_float /= 100.;
    unsigned_float
}

/* -32.768 to +32.768 */
#[inline(always)]
pub fn decodeProtocolSignedThousandthsFloat(b: &[u8]) -> f32 {
    let mut signed_angle = f32::from(dec_prot_i16(b));
    signed_angle /= 1000.;
    signed_angle
}

/* <int16>.<uint16> (-32768.9999 to 32767.9999) */
#[inline(always)]
pub fn decodeProtocol1616Float(b: &[u8]) -> f32 {
    let mut result = dec_prot_i32(b) as f32;
    result /= 65536.;
    result
}

// TODO: Replace with crc crate
pub fn getCRC(message: &[u8], length: u8) -> u8 {
    let mut crc = 0;

    for i in 0..length {
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

pub const CRC7_POLY: u8 = 0x91;
