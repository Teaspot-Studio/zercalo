use glam::f32::Quat;
use glam::{UVec2, UVec3, Vec2, Vec3, Vec4};
use rayon::prelude::*;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn white() -> Self {
        ColorRGB::new(255, 255, 255)
    }

    pub fn black() -> Self {
        ColorRGB::new(0, 0, 0)
    }
}

impl Default for ColorRGB {
    fn default() -> Self {
        ColorRGB::new(0, 0, 0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        ColorRGBA { r, g, b, a }
    }

    pub fn as_vec4(&self) -> Vec4 {
        Vec4::new(
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
            (self.a as f32) / 255.0,
        )
    }

    pub fn empty() -> Self {
        ColorRGBA::new(0, 0, 0, 0)
    }

    pub fn is_empty(&self) -> bool {
        self.a == 0
    }

    pub fn white() -> Self {
        ColorRGBA::new(255, 255, 255, 255)
    }

    pub fn black() -> Self {
        ColorRGBA::new(0, 0, 0, 255)
    }

    pub fn with_alpha(&self, a: u8) -> Self {
        ColorRGBA {
            r: self.r,
            g: self.g,
            b: self.b,
            a,
        }
    }

    /// Convert color to Vec4 where each component are in range 0 .. 1.0 and
    /// premultiplied by alpha value to correctly process alpha blending.
    ///
    /// In that representation RGB components represents how much light of
    /// each channel is captured.
    pub fn as_premultipied(&self) -> Vec4 {
        let x = (self.r as f32) / 255.0;
        let y = (self.g as f32) / 255.0;
        let z = (self.b as f32) / 255.0;
        let w = (self.a as f32) / 255.0;
        Vec4::new(x * w, y * w, z * w, w)
    }

    /// Convert color from Vec4 where each component are in range 0 .. 1.0 and
    /// premultiplied by alpha value to correctly process alpha blending.
    ///
    /// In that representation RGB components represents how much light of
    /// each channel is captured.
    pub fn from_premultiplied(v: &Vec4) -> Self {
        let r = (v.x / v.w * 255.0) as u8;
        let g = (v.y / v.w * 255.0) as u8;
        let b = (v.z / v.w * 255.0) as u8;
        let a = (v.w * 255.0) as u8;
        ColorRGBA::new(r, g, b, a)
    }

    pub fn player1() -> Self {
        ColorRGBA::new(240, 0, 0, 255)
    }

    pub fn player2() -> Self {
        ColorRGBA::new(0, 0, 240, 255)
    }
}

impl Default for ColorRGBA {
    fn default() -> Self {
        ColorRGBA::new(0, 0, 0, 255)
    }
}

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

#[derive(Clone, Debug)]
pub struct Model {
    pub size: UVec3,
    pub voxels: Vec<ColorRGBA>,
    pub offset: Vec3,
    pub rotation: Quat,
    pub replace_colors: HashMap<ColorRGBA, ColorRGBA>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            size: UVec3::new(1, 1, 1),
            voxels: vec![],
            offset: Vec3::ZERO,
            rotation: Quat::from_axis_angle(Vec3::Y, 0.0),
            replace_colors: HashMap::new(),
        }
    }
}

impl Model {
    /// Create new empty model with given size of voxel grid
    pub fn new(size: UVec3) -> Self {
        Model {
            size,
            voxels: vec![ColorRGBA::empty(); size.x as usize * size.y as usize * size.z as usize],
            ..Model::default()
        }
    }

    /// Generate model volume procedurely
    pub fn from_function<F>(size: UVec3, generator: F) -> Self
    where
        F: FnMut(UVec3) -> ColorRGBA + Send + Sync + Clone,
    {
        let mut layers = vec![];
        (0..size.x)
            .into_par_iter()
            .map(|x| {
                let mut columns = vec![];
                (0..size.y)
                    .into_par_iter()
                    .map(|y| {
                        let mut column = vec![ColorRGBA::empty(); size.z as usize];
                        for z in 0..size.z {
                            column[z as usize] = generator.clone()(UVec3::new(x, y, z));
                        }
                        column
                    })
                    .collect_into_vec(&mut columns);
                columns
            })
            .collect_into_vec(&mut layers);

        let voxels: Vec<ColorRGBA> = layers.into_iter().flatten().flatten().collect();
        Model {
            size,
            voxels,
            offset: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_axis_angle(Vec3::Y, 0.0),
            replace_colors: HashMap::new(),
        }
    }

    /// Set voxel color at given local coords, panics on boundary violation
    pub fn set_voxel(&mut self, p: UVec3, v: ColorRGBA) {
        let i = p.x + p.y * self.size.x + p.z * self.size.x * self.size.y;
        self.voxels[i as usize] = v;
    }

    /// Get voxel color at given local coords, panics on boundary violation
    pub fn get_voxel(&self, p: UVec3) -> ColorRGBA {
        let i = p.x + p.y * self.size.x + p.z * self.size.x * self.size.y;
        self.voxels[i as usize]
    }
}

impl Index<UVec3> for Model {
    type Output = ColorRGBA;

    fn index(&self, index: UVec3) -> &Self::Output {
        let i = index.x + index.y * self.size.x + index.z * self.size.x * self.size.y;
        &self.voxels[i as usize]
    }
}

/// Defines distance between each pixel ray. Effectively scales image
pub const DEFAULT_PIXEL_SIZE: f32 = 0.7;
/// Defines maximum distance ray can travel before it considered as going to infininity.
pub const DEFAULT_RAY_MAX_DIST: f32 = 1024.0;
/// Default width of rendered tile
pub const DEFAULT_TILE_WIDTH: u32 = 64;
/// Default height of rendered tile
pub const DEFAULT_TILE_HEIGHT: u32 = 64;

#[derive(Clone, Debug)]
pub struct Camera {
    pub eye: Vec3,
    pub dir: Vec3,
    pub up: Vec3,
    pub pixel_size: f32,
    pub max_dist: f32,
    /// Size of resulted tile in pixels
    pub viewport: UVec2,
    /// How much the tile should be scaled
    pub view_scale: Vec2,
    /// Amount of frames to render
    pub max_frames: u32,
}

impl Default for Camera {
    fn default() -> Self {
        let eye = Vec3::new(32.0, 32.0, 32.0);
        Camera {
            eye,
            dir: -eye.normalize(),
            up: Vec3::new(0.0, 1.0, 0.0),
            pixel_size: DEFAULT_PIXEL_SIZE,
            max_dist: DEFAULT_RAY_MAX_DIST,
            viewport: UVec2::new(DEFAULT_TILE_WIDTH, DEFAULT_TILE_HEIGHT),
            view_scale: Vec2::new(7.0, 7.0),
            max_frames: 128,
        }
    }
}

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
