use anyhow::{Error as AnyError, Result as AnyResult};
use glam::{Quat, Vec3A};
use serde::Deserialize;
use string_cache::DefaultAtom;

use super::{
    avatar::Pose,
    bone::{Bone, Limb},
};

mod wheel;
pub use wheel::{Wheel, WheelConfig};

mod joystick;
pub use joystick::{Joystick, JoystickConfig};

#[derive(Debug, Deserialize)]
#[serde(try_from = "DeviceConfig")]
pub enum Device {
    Wheel(Wheel),
    Joystick(Joystick),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case", tag = "type")]
enum DeviceConfig {
    Wheel(WheelConfig),
    Joystick(JoystickConfig),
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum ForwardPose {
    Global(Quat),
    Local(Quat),
}

impl Device {
    pub fn pose_forward(&self, f: impl FnMut(Bone, f32, ForwardPose)) {
        match *self {
            Device::Wheel(ref w) => w.pose_forward(f),
            Device::Joystick(ref w) => w.pose_forward(f),
        }
    }

    pub fn pose_inverse(&self, pose: &Pose, f: impl FnMut(Limb, f32, Vec3A, Quat)) {
        match *self {
            Device::Wheel(ref w) => w.pose_inverse(pose, f),
            Device::Joystick(ref w) => w.pose_inverse(pose, f),
        }
    }

    pub fn set_value(&mut self, value_x: Option<f32>, value_y: Option<f32>, value_z: Option<f32>) {
        match *self {
            Device::Wheel(ref mut w) => w.set_value(value_x, value_y, value_z),
            Device::Joystick(ref mut w) => w.set_value(value_x, value_y, value_z),
        }
    }

    pub fn trackers(&self, f: impl FnMut(DefaultAtom, Vec3A, Quat)) {
        match *self {
            Device::Wheel(ref w) => w.trackers(f),
            Device::Joystick(ref w) => w.trackers(f),
        }
    }

    pub fn update(&mut self, dt: f64, pose: &Pose) {
        match *self {
            Device::Wheel(ref mut w) => w.update(dt, pose),
            Device::Joystick(ref mut w) => w.update(dt, pose),
        }
    }
}

impl TryFrom<DeviceConfig> for Device {
    type Error = AnyError;

    fn try_from(config: DeviceConfig) -> AnyResult<Self> {
        match config {
            DeviceConfig::Wheel(w) => Ok(Device::Wheel(w.try_into()?)),
            DeviceConfig::Joystick(w) => Ok(Device::Joystick(w.try_into()?)),
        }
    }
}
