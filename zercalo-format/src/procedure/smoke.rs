use glam::f32::Quat;
use glam::{UVec3, Vec3};

use crate::scene::{Model, ColorRGBA};

pub struct SmokePart {
    pub offset: Vec3,
    pub radius: f32,
}

pub struct SmokeModel {
    pub size: UVec3,
    pub offset: Vec3,
    pub rotation: Quat,
    pub particles: Vec<SmokePart>,
}

impl SmokeModel {
    /// Convert smoke to model
    pub fn generate(&self) -> Model {
        let mut model = Model::from_function(self.size, |pos| {
            for part in self.particles.iter() {
                let d2 = (self.offset + part.offset - pos.as_vec3()).length_squared();
                if d2 < part.radius*part.radius {
                    return ColorRGBA::new(150, 150, 150, 255);
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
            part.offset += Vec3::new(0.0, 1.0, 0.0);
            part.radius += 3.0;
        }
    }
}

