use bevy_ecs::prelude::*;
use renderer::ModelInstance;
use crate::component_resources::Time;

#[derive(Debug, Clone, Copy, Component)]
pub struct Transform {
    previous_translation: glam::Vec3,
    previous_rotation: glam::Quat,
    previous_scale: glam::Vec3,

    interpolated_translation: glam::Vec3,
    interpolated_rotation: glam::Quat,
    interpolated_scale: glam::Vec3,

    translation: glam::Vec3,
    rotation: glam::Quat,
    scale: glam::Vec3,
}

impl From<&Transform> for ModelInstance {
    fn from(value: &Transform) -> Self {
        ModelInstance {
            scale: value.interpolated_scale,
            rotation: value.interpolated_rotation,
            translation: value.interpolated_translation,
        }
    }
}

impl From<Transform> for ModelInstance {
    fn from(value: Transform) -> Self {
        ModelInstance {
            scale: value.interpolated_scale,
            rotation: value.interpolated_rotation,
            translation: value.interpolated_translation,
        }
    }
}

impl Transform {
    pub fn copy_forward_all(mut query: Query<&mut Transform>) {
        for mut transform in &mut query {
            transform.copy_forward()
        }
    }

    pub fn interpolate_all(mut query: Query<&mut Transform>, timing: Res<Time>) {
        for mut transform in &mut query {
            transform.interpolate(timing.alpha())
        }
    }

    pub fn new(translation: glam::Vec3, rotation: glam::Quat, scale: glam::Vec3) -> Self {
        Self {
            previous_translation: translation,
            previous_rotation: rotation,
            previous_scale: scale,
            interpolated_translation: translation,
            interpolated_rotation: rotation,
            interpolated_scale: scale,
            translation,
            rotation,
            scale,
        }
    }

    pub fn rotate(&mut self, horizontal: f32, vertical: f32, pitch_limit: Option<f32>) {
        let (mut yaw, mut pitch, roll) = self.rotation.to_euler(glam::EulerRot::YXZ);

        yaw -= horizontal;
        pitch += vertical;

        if let Some(pitch_limit) = pitch_limit {
            pitch = pitch.clamp(-pitch_limit + 0.001, pitch_limit - 0.001);
        }

        if yaw > std::f32::consts::PI {
            yaw = std::f32::consts::PI;
        } else if yaw < -std::f32::consts::PI {
            yaw = -std::f32::consts::PI;
        }

        self.rotation = glam::Quat::from_euler(glam::EulerRot::YXZ, yaw, pitch, roll);
    }

    pub fn interpolate(&mut self, alpha: f32) {
        self.interpolated_translation = self.previous_translation.lerp(self.translation, alpha);
        self.interpolated_rotation = self.previous_rotation.slerp(self.rotation, alpha);
        self.interpolated_scale = self.previous_scale.lerp(self.scale, alpha);
    }

    pub fn copy_forward(&mut self) {
        self.previous_translation = self.translation;
        self.previous_rotation = self.rotation;
        self.previous_scale = self.scale;
    }

    pub fn translation(&self) -> &glam::Vec3 {
        &self.translation
    }

    pub fn translation_mut(&mut self) -> &mut glam::Vec3 {
        &mut self.translation
    }

    pub fn rotation(&self) -> &glam::Quat {
        &self.rotation
    }

    pub fn scale(&self) -> &glam::Vec3 {
        &self.scale
    }
}