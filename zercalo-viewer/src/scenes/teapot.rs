use glam::{UVec2, Vec2, Vec3};
use zercalo_format::animation::RotationView;
use zercalo_format::scene::{Camera, ColorRGB, Light, Scene};
use zercalo_format::import::vox::{from_vox_file, VoxImportError};

pub type TeapotScene = RotationView<Scene>;

pub fn new_teapot_scene() -> Result<TeapotScene, VoxImportError> {
    let model = from_vox_file("./assets/models/teapot.vox")?;

    let eye = Vec3::new(128., 128., 128.);
    let scene = Scene {
        camera: Camera {
            eye,
            dir: -eye.normalize(),
            pixel_size: 1.0,
            viewport: UVec2::new(256, 256),
            view_scale: Vec2::new(2.0, 2.0),
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
        target_y: Some(32.0),
        rotation_speed: std::f32::consts::PI / 180.0,
    })
}