use std::f32::consts::TAU;

use glam::{EulerRot, Quat, Vec3A};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

use crate::vmc::{
    avatar::Pose,
    bone::{Bone, Limb},
    device::{ForwardPose, Throttle},
};

#[derive(Debug)]
pub struct Technique {
    left_hand_angle: f32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct TechniqueConfig {
    #[serde(default = "left_hand_default", deserialize_with = "parse_position")]
    left_hand: f32,
}

fn left_hand_default() -> f32 {
    0.5 * TAU
}

impl Technique {
    pub fn pose_forward(&self, _: &Throttle, mut f: impl FnMut(Bone, f32, ForwardPose)) {
        let bones = [
            (Bone::LeftIndexProximal, Quat::from_rotation_z(0.05 * TAU)),
            (
                Bone::LeftIndexIntermediate,
                Quat::from_rotation_z(0.25 * TAU),
            ),
            (Bone::LeftIndexDistal, Quat::from_rotation_z(0.05 * TAU)),
            (Bone::LeftMiddleProximal, Quat::from_rotation_z(0.05 * TAU)),
            (
                Bone::LeftMiddleIntermediate,
                Quat::from_rotation_z(0.25 * TAU),
            ),
            (Bone::LeftMiddleDistal, Quat::from_rotation_z(0.05 * TAU)),
            (Bone::LeftRingProximal, Quat::from_rotation_z(0.05 * TAU)),
            (
                Bone::LeftRingIntermediate,
                Quat::from_rotation_z(0.25 * TAU),
            ),
            (Bone::LeftRingDistal, Quat::from_rotation_z(0.05 * TAU)),
            (Bone::LeftLittleProximal, Quat::from_rotation_z(0.05 * TAU)),
            (
                Bone::LeftLittleIntermediate,
                Quat::from_rotation_z(0.25 * TAU),
            ),
            (Bone::LeftLittleDistal, Quat::from_rotation_z(0.05 * TAU)),
        ];

        for (bone, rot) in bones {
            f(bone, 1.0, ForwardPose::Local(rot));
        }
    }

    pub fn pose_inverse(&self, _: &Pose, throttle: &Throttle, mut f: impl FnMut(Limb, f32, Vec3A, Quat)) {
        let hands = [
            (Limb::LeftHand, self.left_hand_angle, 0.25 * TAU),
        ];

        for (limb, _angle, yaw) in hands {
            let local_pos = Vec3A::from((0.01, 1.0 * throttle.radius, -0.04866177));
            let local_rot = Quat::from_euler(EulerRot::YXZ, yaw, 0.0, 0.0);
            f(
                limb,
                1.0,
                throttle.pos + throttle.rot * local_pos,
                throttle.rot * local_rot,
            );
        }
    }

    pub fn update(&mut self, _: f64, _: &Pose) {}
}

fn parse_position<'de, D: Deserializer<'de>>(de: D) -> Result<f32, D::Error> {
    let raw = f32::deserialize(de)?;
    (0.0..=360.0)
        .contains(&raw)
        .then(|| f32::to_radians(90.0 - raw).rem_euclid(TAU))
        .ok_or_else(|| {
            D::Error::invalid_value(
                Unexpected::Float(raw as f64),
                &"an angle in the range [0, 360]",
            )
        })
}

impl From<TechniqueConfig> for Technique {
    fn from(config: TechniqueConfig) -> Self {
        Technique {
            left_hand_angle: config.left_hand,
        }
    }
}

impl Default for TechniqueConfig {
    fn default() -> Self {
        TechniqueConfig {
            left_hand: 0.5 * TAU,
        }
    }
}
