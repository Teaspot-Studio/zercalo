pub mod camera;
pub mod getters;
pub mod light;
pub mod model;

pub use camera::*;
pub use getters::*;
pub use light::*;
pub use model::*;

use crate::color::ColorRGB;
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct Scene {
    pub models: Vec<Model>,
    pub lights: Vec<Light>,
    pub camera: Camera,
    pub ambient: ColorRGB,
}

impl Scene {
    /// Get bounding volume of all scene
    pub fn bounding(&self) -> (Vec3, Vec3) {
        let minv = std::f32::MIN;
        let maxv = std::f32::MAX;
        let mut max_vec = Vec3::new(minv, minv, minv);
        let mut min_vec = Vec3::new(maxv, maxv, maxv);

        for m in self.models.iter() {
            min_vec = min_vec.min(m.offset);
            max_vec = max_vec.max(m.offset + m.rotation.mul_vec3(m.size.as_vec3()));
        }
        (min_vec, max_vec)
    }

    /// Get center of bounding volume of all scene
    pub fn center(&self) -> Vec3 {
        let (minv, maxv) = self.bounding();
        (maxv - minv) * 0.5
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            models: vec![],
            lights: vec![Light::default()],
            camera: Camera::default(),
            ambient: ColorRGB::new(25, 25, 25),
        }
    }
}
