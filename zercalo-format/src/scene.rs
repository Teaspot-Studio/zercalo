use glam::{UVec3, Vec3};

pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ColorRGB { r, g, b }
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
        )
    }
}

pub struct Light {
    pub position: Vec3,
    pub color: ColorRGB,
}

impl Default for Light {
    fn default() -> Self {
        Light {
            position: Vec3::new(25.0, 25.0, 25.0),
            color: ColorRGB::new(255, 255, 255),
        }
    }
}

pub struct Model {
    pub size: UVec3,
}

/// Defines distance between each pixel ray. Effectively scales image
const DEFAULT_PIXEL_SIZE: f32 = 0.7;

pub struct Camera {
    pub eye: Vec3,
    pub dir: Vec3,
    pub up: Vec3,
    pub pixel_size: f32,
}

impl Default for Camera {
    fn default() -> Self {
        let eye = Vec3::new(32.0, 32.0, 32.0);
        Camera {
            eye,
            dir: -eye.normalize(),
            up: Vec3::new(0.0, 1.0, 0.0),
            pixel_size: DEFAULT_PIXEL_SIZE,
        }
    }
}

pub struct Scene {
    pub models: Vec<Model>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            models: vec![],
            lights: vec![Light::default()],
            camera: Camera::default(),
        }
    }
}
