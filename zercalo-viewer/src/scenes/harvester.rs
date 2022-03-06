use glam::{UVec2, Vec2, Vec3};
use maplit::hashmap;
use zercalo_format::animation::RotationView;
use zercalo_format::import::vox::{from_vox_file, VoxImportError};
use zercalo_format::scene::{Camera, ColorRGB, Light, Scene, ColorRGBA};

pub type HarvesterScene = RotationView<Scene>;

pub fn new_harvester_scene(player_color: ColorRGBA) -> Result<HarvesterScene, VoxImportError> {
    let mut model = from_vox_file("./assets/models/harvester_full.vox")?[0].clone();
    model.replace_colors = hashmap!{
        ColorRGBA::new(183, 183, 183, 255) => player_color,
        ColorRGBA::new(23, 84, 131, 255) => ColorRGBA::new(23, 84, 131, 100),
    };

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
        models: vec![model],
        ..Scene::default()
    };
    Ok(RotationView {
        scene,
        target_y: Some(8.0),
        rotation_speed: std::f32::consts::PI / 180.0,
    })
}