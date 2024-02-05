use std::f32::consts::TAU;

use glam::{EulerRot, Quat, Vec3A};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer,
};

use crate::vmc::{
    avatar::Pose,
    bone::{Bone, Limb},
    device::{ForwardPose, Joystick},
};

#[derive(Debug)]
pub struct Technique {
    right_hand_angle: f32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct TechniqueConfig {
    #[serde(default = "right_hand_default", deserialize_with = "parse_position")]
    right_hand: f32,
}

fn right_hand_default() -> f32 {
    0.0 * TAU
}

impl Technique {
    pub fn pose_forward(&self, _: &Joystick, mut f: impl FnMut(Bone, f32, ForwardPose)) {
        let bones = [
            (Bone::RightIndexProximal, Quat::from_rotation_z(-0.1 * TAU)),
            (
                Bone::RightIndexIntermediate,
                Quat::from_rotation_z(-0.25 * TAU),
            ),
            (Bone::RightIndexDistal, Quat::from_rotation_z(-0.1 * TAU)),
            (Bone::RightMiddleProximal, Quat::from_rotation_z(-0.1 * TAU)),
            (
                Bone::RightMiddleIntermediate,
                Quat::from_rotation_z(-0.25 * TAU),
            ),
            (Bone::RightMiddleDistal, Quat::from_rotation_z(-0.1 * TAU)),
            (Bone::RightRingProximal, Quat::from_rotation_z(-0.1 * TAU)),
            (
                Bone::RightRingIntermediate,
                Quat::from_rotation_z(-0.25 * TAU),
            ),
            (Bone::RightRingDistal, Quat::from_rotation_z(-0.1 * TAU)),
            (Bone::RightLittleProximal, Quat::from_rotation_z(-0.1 * TAU)),
            (
                Bone::RightLittleIntermediate,
                Quat::from_rotation_z(-0.25 * TAU),
            ),
            (Bone::RightLittleDistal, Quat::from_rotation_z(-0.1 * TAU)),
            (
                Bone::RightThumbProximal,
                Quat::from_euler(EulerRot::YZX, 0.02 * TAU, 0.0, 0.08 * TAU),
            ),
            (
                Bone::RightThumbIntermediate,
                Quat::from_rotation_y(0.08 * TAU),
            ),
            (Bone::RightThumbDistal, Quat::from_rotation_y(0.03 * TAU)),
        ];

        for (bone, rot) in bones {
            f(bone, 1.0, ForwardPose::Local(rot));
        }
    }

    pub fn pose_inverse(&self, _: &Pose, joystick: &Joystick, mut f: impl FnMut(Limb, f32, Vec3A, Quat)) {
        let hands = [
            (Limb::RightHand, self.right_hand_angle, -0.25 * TAU),
        ];

        for (limb, angle, yaw) in hands {
            let local_pos = Vec3A::from((0.02, 1.0 * joystick.radius, -0.055));
            let local_rot = Quat::from_euler(EulerRot::YXZ, yaw, yaw - yaw.signum() * angle, 0.0);
            f(
                limb,
                1.0,
                joystick.pos + joystick.rot * local_pos,
                joystick.rot * local_rot,
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
            right_hand_angle: config.right_hand,
        }
    }
}

impl Default for TechniqueConfig {
    fn default() -> Self {
        TechniqueConfig {
            right_hand: 0.0 * TAU,
        }
    }
}
