use anyhow::{ensure, Context as _, Error as AnyError, Result as AnyResult};
use glam::{EulerRot, Quat, Vec3A};
use serde::Deserialize;
use string_cache::DefaultAtom;

mod technique;

use crate::vmc::{
    avatar::Pose,
    bone::{Bone, Limb},
};
use technique::{Technique, TechniqueConfig};

use super::ForwardPose;

#[derive(Debug)]
pub struct Throttle {
    pos: Vec3A,
    rot: Quat,
    radius: f32,
    angle_x: f32,
    angle_y: f32,
    angle_z: f32,
    base_rot: Quat,
    tracker: Option<DefaultAtom>,
    technique: Technique,
}

#[derive(Debug, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct ThrottleConfig {
    position: [f32; 3],
    rotation: [f32; 3],
    radius: f32,
    tracker: Option<DefaultAtom>,
    technique: TechniqueConfig,
}

impl TryFrom<ThrottleConfig> for Throttle {
    type Error = AnyError;

    fn try_from(config: ThrottleConfig) -> AnyResult<Self> {
        ensure!(config.radius > 0.0, "Wheel radius must be positive.");

        let rot = Quat::from_euler(
            EulerRot::YXZ,
            config.rotation[1].to_radians(),
            config.rotation[0].to_radians(),
            config.rotation[2].to_radians(),
        );

        Ok(Throttle {
            pos: config.position.into(),
            radius: config.radius,
            rot,
            angle_x: 0.0,
            angle_y: 0.0,
            angle_z: 0.0,
            base_rot: rot,
            tracker: config.tracker,
            technique: config
                .technique
                .try_into()
                .context("Failed to initialize driving throttle handling technique")?,
        })
    }
}

impl Default for ThrottleConfig {
    fn default() -> ThrottleConfig {
        ThrottleConfig {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            radius: 0.17,
            tracker: None,
            technique: TechniqueConfig::default(),
        }
    }
}

impl Throttle {
    pub fn pose_forward(&self, f: impl FnMut(Bone, f32, ForwardPose)) {
        self.technique.pose_forward(self, f)
    }

    pub fn pose_inverse(&self, pose: &Pose, f: impl FnMut(Limb, f32, Vec3A, Quat)) {
        self.technique.pose_inverse(pose, self, f)
    }

    pub fn set_value(&mut self, value_x: Option<f32>, value_y: Option<f32>, value_z: Option<f32>) {
        match value_x {
            Some(new_value) => self.angle_x = new_value,
            None => ()
        };

        match value_y {
            Some(new_value) => self.angle_y = new_value,
            None => ()
        };

        match value_z {
            Some(new_value) => self.angle_z = new_value,
            None => ()
        };

        self.rot = self.base_rot * Quat::from_rotation_x(-self.angle_x.to_radians()) * Quat::from_rotation_y(-self.angle_y.to_radians()) * Quat::from_rotation_z(-self.angle_z.to_radians());
    }

    pub fn trackers(&self, mut f: impl FnMut(DefaultAtom, Vec3A, Quat)) {
        if let Some(ref tracker) = self.tracker.clone() {
            f(tracker.clone(), self.pos, self.rot);
        }
    }

    pub fn update(&mut self, dt: f64, pose: &Pose) {
        self.technique.update(dt, pose)
    }
}
