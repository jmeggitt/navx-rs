// Copyright 2018 navx-rs Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*****************************************************************************/
/* This protocol, introduced first with the navX MXP, expands upon the IMU   */
/* protocol by adding the following new functionality:                       */
/*         																	 */
/* AHRS Update:  Includes Fused Heading and Altitude Info                    */
/* Magnetometer Calibration:  Enables configuration of coefficients from PC  */
/* Board Identity:  Enables retrieval of Board Identification Info           */
/* Fusion Tuning:  Enables configuration of key thresholds/coefficients used */
/*                 in data fusion algorithms from a remote client            */
/*                                                                           */
/* In addition, the navX enable stream command has been extended with a new  */
/* Stream type, in order to enable AHRS Updates.                             */
/*****************************************************************************/


#[derive(Debug, Clone, Default)]
pub struct AHRSUpdateBase {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub compass_heading: f32,
    pub altitude: f32,
    pub fused_heading: f32,
    pub linear_accel_x: f32,
    pub linear_accel_y: f32,
    pub linear_accel_z: f32,
    pub mpu_temp: f32,
    pub quat_w: f32,
    pub quat_x: f32,
    pub quat_y: f32,
    pub quat_z: f32,
    pub barometric_pressure: f32,
    pub baro_temp: f32,
    pub op_status: u8,
    pub sensor_status: u8,
    pub cal_status: u8,
    pub selftest_status: u8,
}


#[derive(Debug, Default, Clone)]
pub struct AHRSUpdate {
    pub base: AHRSUpdateBase,
    pub cal_mag_x: i16,
    pub cal_mag_y: i16,
    pub cal_mag_z: i16,
    pub mag_field_norm_ratio: f32,
    pub mag_field_norm_scalar: f32,
    pub raw_mag_x: i16,
    pub raw_mag_y: i16,
    pub raw_mag_z: i16,
}


#[derive(Debug, Default, Clone)]
pub struct AHRSPosUpdate {
    pub base: AHRSUpdateBase,
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
    pub disp_x: f32,
    pub disp_y: f32,
    pub disp_z: f32,
}


#[derive(Debug, Default, Clone)]
pub struct AHRSPosTSUpdate {
    pos_upd: AHRSPosUpdate,
    timestamp: u32,
}


#[derive(Debug, Default, Clone)]
pub struct BoardID {
    pub type_: u8,
    pub hw_rev: u8,
    pub fw_ver_major: u8,
    pub fw_ver_minor: u8,
    pub fw_revision: i16,
    pub unique_id: [u8; 12],
}


#[derive(Debug, Default, Clone)]
pub struct IntegrationControl {
    action: u8,
    parameter: i32,
}


#[derive(Debug, Default, Clone)]
pub struct YPRUpdate {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub compass_heading: f32,
}


#[derive(Debug, Default, Clone)]
pub struct GyroUpdate {
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub mag_x: i16,
    pub mag_y: i16,
    pub mag_z: i16,
    pub temp_c: f32,
}
