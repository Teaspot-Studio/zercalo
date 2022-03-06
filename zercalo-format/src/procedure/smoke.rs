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
    pub very_hot_color: ColorRGBA,
    pub ceiling_height: f32,
    pub ceiling_speed: f32, // how fast parts shrinks after ceiling
}

impl Default for SmokeModel {
    fn default() -> Self {
        SmokeModel {
            size: UVec3::new(1, 1, 1),
            offset: Vec3::ZERO,
            rotation: Quat::from_axis_angle(Vec3::Y, 0.0),
            particles: vec![],
            noise: OpenSimplex::new(),
            cold_color: ColorRGBA::new(111, 123, 155, 255),
            hot_color: ColorRGBA::new(229, 88, 41, 255),
            very_hot_color: ColorRGBA::new(249, 195, 0, 255),
            ceiling_height: 42.0,
            ceiling_speed: -0.1,
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
                let d2 = (part.offset - pos.as_vec3()).length_squared();

                let dr = (self.noise.get([
                    (pos.x as f64) * part.scale_noise_coords.x as f64,
                    (pos.y as f64) * part.scale_noise_coords.y as f64,
                    (pos.z as f64) * part.scale_noise_coords.z as f64,
                ]) * part.scale_noise_result as f64) as f32;

                let dr2 = d2 + dr;
                if dr2 < part.radius * part.radius && dr2 > 0.0 && part.radius > 0.0 {
                    let cold_radius = part.temperature * part.radius;
                    let very_hot_radius = cold_radius * 0.7;
                    if part.offset.y > self.ceiling_height {
                        return self.cold_color;
                    } else if dr2 < very_hot_radius * very_hot_radius {
                        return self.very_hot_color;
                    } else if dr2 < cold_radius * cold_radius {
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
            if part.offset.y > self.ceiling_height {
                part.radius += self.ceiling_speed;
                if part.radius < 0.0 {
                    part.radius = 0.0;
                }
            } else {
                part.radius += part.radius_vel;
            }
            part.temperature += part.temperature_speed;
        }
    }
}
