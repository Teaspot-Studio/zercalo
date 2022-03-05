use glam::Vec3;
use zercalo_format::animation::{HasBounding, HasMutCamera, Renderable, RotationView};
use zercalo_format::procedure::smoke::SmokeModel;
use zercalo_format::scene::{Camera, ColorRGB, Light, Scene};

pub struct TestScene {
    smoke: SmokeModel,
    /// Scene is cached to store voxels for renderer
    rendered: Scene,
}

impl TestScene {
    pub fn new() -> RotationView<TestScene> {
        let model = SmokeModel::default();
        let eye = Vec3::new(128., 128., 128.);
        let mut scene = TestScene {
            smoke: model,
            rendered: Scene {
                camera: Camera {
                    eye,
                    dir: -eye.normalize(),
                    ..Camera::default()
                },
                lights: vec![Light {
                    position: Vec3::new(128.0, 150.0, 75.0),
                    color: ColorRGB::white(),
                }],
                ..Scene::default()
            },
        };
        scene.animate(0);
        RotationView {
            scene,
            target_y: Some(32.0),
            rotation_speed: 0.0,
        } // std::f32::consts::PI / 180.0
    }
}

impl HasMutCamera for TestScene {
    fn get_mut_camera(&'_ mut self) -> &'_ mut Camera {
        &mut self.rendered.camera
    }
}

impl Renderable for TestScene {
    fn animate(&mut self, frame: u32) {
        self.smoke.animate(frame);
        self.rendered.models = vec![self.smoke.generate()];
    }

    fn render(&self) -> &Scene {
        &self.rendered
    }
}

impl HasBounding for TestScene {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.rendered.bounding()
    }
}
