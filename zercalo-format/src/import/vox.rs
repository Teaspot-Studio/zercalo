use crate::scene::{ColorRGBA, Model};
use glam::f32::Quat;
use glam::{UVec3, Vec3};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VoxImportError {
    #[error("Failed to import VOX file: {0}")]
    Vox(String),
    #[error("Failed to open file: {0}")]
    File(#[from] std::io::Error),
}

/// Reads a VOX file from a slice into [`crate::scene::Model`]
pub fn from_vox_slice(slice: &[u8]) -> Result<Vec<Model>, VoxImportError> {
    let voxdata = dot_vox::load_bytes(slice).map_err(|e| VoxImportError::Vox(e.to_owned()))?;
    let models = voxdata
        .models
        .iter()
        .map(|m| from_vox_model(&voxdata.palette, m))
        .collect();
    Ok(models)
}

/// Reads a VOX file from the specified path into [`crate::scene::Model`]
pub fn from_vox_file(path: &str) -> Result<Vec<Model>, VoxImportError> {
    let voxdata = dot_vox::load(path).map_err(|e| VoxImportError::Vox(e.to_owned()))?;
    Ok(from_vox_data(voxdata))
}

/// Reads a VOX file from parsed in memory data into [`crate::scene::Model`]
pub fn from_vox_data(data: dot_vox::DotVoxData) -> Vec<Model> {
    data.models
        .iter()
        .map(|m| from_vox_model(&data.palette, m))
        .collect()
}

/// Import parsed VOX model with given pallete to own model format
pub fn from_vox_model(pallete: &[u32], vox_model: &dot_vox::Model) -> Model {
    let size = UVec3::new(vox_model.size.x, vox_model.size.z, vox_model.size.y);
    let mut voxels = vec![ColorRGBA::empty(); (size.x * size.y * size.z) as usize];
    for v in vox_model.voxels.iter() {
        let i = v.x as u32 + v.z as u32 * size.x + v.y as u32 * size.x * size.y;
        voxels[i as usize] = vox_color_to_rgba(pallete[v.i as usize]);
    }
    Model {
        size,
        voxels,
        offset: Vec3::new(0.0, 0.0, 0.0),
        rotation: Quat::from_axis_angle(Vec3::Y, 0.0),
    }
}

#[inline]
pub fn vox_color_to_rgba(c: u32) -> ColorRGBA {
    ColorRGBA::new(
        (c & 0xFF) as u8,
        ((c >> 4) & 0xFF) as u8,
        ((c >> 8) & 0xFF) as u8,
        ((c >> 16) & 0xFF) as u8,
    )
}
