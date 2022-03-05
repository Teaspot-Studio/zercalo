use glam::{UVec2, Vec2, Vec3};
use zercalo_format::animation::RotationView;
use zercalo_format::scene::{Camera, ColorRGB, Light, Scene};
use zercalo_format::import::vox::{from_vox_file, VoxImportError};

pub type HarvesterScene = RotationView<Scene>;

pub fn new_harvester_scene() -> Result<HarvesterScene, VoxImportError> {
    let model = from_vox_file("./assets/models/harvester_full.vox")?;

    let eye = Vec3::new(128., 128., 128.);
    let scene = Scene {
        camera: Camera {
            eye,
            dir: -eye.normalize(),
            pixel_size: 0.5,
            viewport: UVec2::new(128, 128),
            view_scale: Vec2::new(7.0, 7.0),
            ..Camera::default()
        },
        lights: vec![Light {
            position: Vec3::new(128.0, 150.0, 75.0),
            color: ColorRGB::white(),
        }],
        models: vec![model[0].clone()],
        ..Scene::default()
    };
    Ok(RotationView {
        scene,
        target_y: Some(8.0),
        rotation_speed: std::f32::consts::PI / 180.0,
    })
}