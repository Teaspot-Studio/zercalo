use crate::color::ColorRGBA;
use glam::f32::Quat;
use glam::{UVec3, Vec3};
use rayon::prelude::*;
use std::collections::HashMap;
use std::ops::Index;

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
