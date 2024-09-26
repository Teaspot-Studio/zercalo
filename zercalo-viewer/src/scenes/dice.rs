use glam::{UVec2, UVec3, Vec2, Vec3};
use zercalo_format::animation::{Animatable, RotationView};
use zercalo_format::color::{ColorRGB, ColorRGBA};
use zercalo_format::import::vox::{from_vox_file, VoxImportError};
use zercalo_format::procedure::particles::ParticlesModel;
use zercalo_format::scene::{Camera, HasBounding, HasCamera, HasMutCamera, HasScene, Light, Scene};

pub struct DiceScene {
    /// Scene is cached to store voxels for renderer
    rendered: Scene,
}

impl DiceScene {
    pub fn new() -> Result<RotationView<Self>, VoxImportError> {
        let model = from_vox_file("./assets/models/dice/dice_orig.vox")?;

        // let rng = fastrand::Rng::with_seed(42);

        let eye = Vec3::new(256., 256., 256.);
        let scene = Scene {
            camera: Camera {
                eye,
                dir: -eye.normalize(),
                pixel_size: 0.4,
                viewport: UVec2::new(128, 128),
                view_scale: Vec2::new(4.0, 4.0),
                max_frames: 420,
                ..Camera::default()
            },
            lights: vec![Light {
                position: Vec3::new(128.0, 150.0, 75.0),
                color: ColorRGB::white(),
            }],
            models: vec![model[0].clone()],
            ..Scene::default()
        };
        let mut dice_scene = DiceScene {
            rendered: scene,
        };
        dice_scene.animate(0);
        Ok(RotationView {
            scene: dice_scene,
            target_y: Some(8.0),
            rotation_speed: 1. * std::f32::consts::PI / 180.0,
        }) // std::f32::consts::PI / 180.0
    }
}

impl HasCamera for DiceScene {
    fn get_camera(&self) -> &Camera {
        &self.rendered.camera
    }
}

impl HasMutCamera for DiceScene {
    fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.rendered.camera
    }
}

impl HasScene for DiceScene {
    fn get_scene(&self) -> &Scene {
        &self.rendered
    }
}

impl Animatable for DiceScene {
    fn animate(&mut self, frame: u32) {
        // self.sand.animate(frame);
        // self.rendered.models = vec![self.sand.generate()];
    }
}

impl HasBounding for DiceScene {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.rendered.bounding()
    }
}
