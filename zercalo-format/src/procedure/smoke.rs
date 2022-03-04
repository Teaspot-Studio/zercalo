use glam::f32::Quat;
use glam::{UVec3, Vec3};
use noise::{NoiseFn, OpenSimplex};

use crate::animation::HasBounding;
use crate::scene::{ColorRGBA, Model};

pub struct SmokePart {
    pub offset: Vec3,
    pub radius: f32,
    pub velocity: Vec3,
    pub radius_vel: f32,
    pub temperature: f32,
    pub temperature_speed: f32,
    pub scale_noise_coords: Vec3,
    pub scale_noise_result: f32,
}

pub struct SmokeModel {
    pub size: UVec3,
    pub offset: Vec3,
    pub rotation: Quat,
    pub particles: Vec<SmokePart>,
    pub noise: OpenSimplex,
    pub cold_color: ColorRGBA,
    pub hot_color: ColorRGBA,
}

impl Default for SmokeModel {
    fn default() -> Self {
        SmokeModel {
            size: UVec3::new(16, 16, 16),
            offset: Vec3::ZERO,
            rotation: Quat::from_axis_angle(Vec3::Y, 0.0),
            particles: vec![],
            noise: OpenSimplex::new(),
            cold_color: ColorRGBA::new(111, 123, 155, 255),
            hot_color: ColorRGBA::new(229, 88, 41, 255),
        }
    }
}

impl HasBounding for SmokeModel {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        (
            self.offset,
            self.offset + self.rotation.mul_vec3(self.size.as_vec3()),
        )
    }
}

impl SmokeModel {
    /// Create new empty model for smoke
    pub fn new() -> Self {
        SmokeModel::default()
    }

    /// Convert smoke to model
    pub fn generate(&self) -> Model {
        let mut model = Model::from_function(self.size, |pos| {
            for part in self.particles.iter() {
                let d2 = (self.offset + part.offset - pos.as_vec3()).length_squared();
                let dr = (self.noise.get([
                    (pos.x as f64) * part.scale_noise_coords.x as f64,
                    (pos.y as f64) * part.scale_noise_coords.y as f64,
                    (pos.z as f64) * part.scale_noise_coords.z as f64,
                ]) * part.scale_noise_result as f64) as f32;

                if d2 + dr < part.radius * part.radius {
                    let cold_radius = part.temperature * part.radius;
                    if d2 + dr < cold_radius * cold_radius {
                        return self.hot_color;
                    } else {
                        return self.cold_color;
                    }
                }
            }
            ColorRGBA::empty()
        });
        model.rotation = self.rotation;
        model.offset = self.offset;
        model
    }

    /// Step animation of smoke
    pub fn animate(&mut self, _frame: u32) {
        for part in self.particles.iter_mut() {
            part.offset += part.velocity;
            part.radius += part.radius_vel;
            part.temperature += part.temperature_speed;
        }
    }
}
