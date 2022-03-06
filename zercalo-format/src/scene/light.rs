use crate::color::ColorRGB;
use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Light {
    pub position: Vec3,
    pub color: ColorRGB,
}

impl Default for Light {
    fn default() -> Self {
        Light {
            position: Vec3::new(23.0, 25.0, 27.0),
            color: ColorRGB::new(255, 255, 255),
        }
    }
}
