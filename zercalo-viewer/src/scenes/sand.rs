use glam::{UVec2, UVec3, Vec2, Vec3};
use zercalo_format::animation::{Renderable, RotationView};
use zercalo_format::color::{ColorRGB, ColorRGBA};
use zercalo_format::procedure::particles::ParticlesModel;
use zercalo_format::scene::{Camera, HasBounding, HasCamera, HasMutCamera, Light, Scene};

pub struct SandScene {
    sand: ParticlesModel,
    /// Scene is cached to store voxels for renderer
    rendered: Scene,
}

impl SandScene {
    pub fn new() -> RotationView<Self> {
        let rng = fastrand::Rng::with_seed(42);
        let colors = vec![
            ColorRGBA::new(242, 183, 106, 100),
            ColorRGBA::new(232, 198, 150, 100),
            ColorRGBA::new(255, 145, 56, 100),
        ];
        let mut model = ParticlesModel::new_random(
            &rng,
            UVec3::new(128, 128, 128),
            (500, 700),
            (Vec3::new(0.0, 0.1, 0.0), Vec3::new(0.3, 1.0, 0.3)),
            (Vec3::new(60.0, -20.0, 60.0), Vec3::new(70.0, 1.0, 70.0)),
            (1, 3),
            &colors,
        );
        model.gravity = Vec3::new(0.0, -0.007, 0.0);

        let eye = Vec3::new(256., 256., 256.);
        let scene = Scene {
            camera: Camera {
                eye,
                dir: -eye.normalize(),
                pixel_size: 1.0,
                viewport: UVec2::new(128, 128),
                view_scale: Vec2::new(4.0, 4.0),
                max_frames: 420,
                ..Camera::default()
            },
            lights: vec![Light {
                position: Vec3::new(128.0, 150.0, 75.0),
                color: ColorRGB::white(),
            }],
            ..Scene::default()
        };
        let mut sand_scene = SandScene {
            sand: model,
            rendered: scene,
        };
        sand_scene.animate(0);
        RotationView {
            scene: sand_scene,
            target_y: Some(0.0),
            rotation_speed: 0.0,
        } // std::f32::consts::PI / 180.0
    }
}

impl HasCamera for SandScene {
    fn get_camera(&self) -> &Camera {
        &self.rendered.camera
    }
}

impl HasMutCamera for SandScene {
    fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.rendered.camera
    }
}

impl Renderable for SandScene {
    fn animate(&mut self, frame: u32) {
        self.sand.animate(frame);
        self.rendered.models = vec![self.sand.generate()];
    }

    fn render(&self) -> &Scene {
        &self.rendered
    }
}

impl HasBounding for SandScene {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.rendered.bounding()
    }
}
