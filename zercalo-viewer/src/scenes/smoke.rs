use glam::f32::Quat;
use glam::{UVec2, UVec3, Vec3};
use noise::OpenSimplex;
use zercalo_format::animation::{Animatable, RotationView};
use zercalo_format::color::{ColorRGB, ColorRGBA};
use zercalo_format::procedure::smoke::{SmokeModel, SmokePart};
use zercalo_format::scene::{Camera, HasBounding, HasCamera, HasMutCamera, HasScene, Light, Scene};

pub struct SmokeScene {
    smoke: SmokeModel,
    /// Scene is cached to store voxels for renderer
    rendered: Scene,
}

impl SmokeScene {
    pub fn new() -> RotationView<SmokeScene> {
        let model = SmokeScene::smoke_model();
        let eye = Vec3::new(128., 128., 128.);
        let mut scene = SmokeScene {
            smoke: model,
            rendered: Scene {
                camera: Camera {
                    eye,
                    dir: -eye.normalize(),
                    viewport: UVec2::new(64, 128),
                    max_frames: 512,
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

    fn smoke_model() -> SmokeModel {
        let particles = vec![
            SmokePart {
                offset: Vec3::new(32.0, 2.0, 32.0),
                radius: 3.0,
                velocity: Vec3::new(0.0, 0.2, 0.0),
                radius_vel: 0.02,
                temperature: 1.0,
                temperature_speed: -0.001,
                scale_noise_coords: Vec3::new(0.2, 0.2, 0.2),
                scale_noise_result: 40.0,
            },
            SmokePart {
                offset: Vec3::new(25.0, -5.0, 32.0),
                radius: 2.0,
                velocity: Vec3::new(0.0, 0.2, 0.0),
                radius_vel: 0.02,
                temperature: 1.0,
                temperature_speed: -0.002,
                scale_noise_coords: Vec3::new(0.2, 0.2, 0.2),
                scale_noise_result: 50.0,
            },
            SmokePart {
                offset: Vec3::new(32.0, -5.0, 25.0),
                radius: 5.0,
                velocity: Vec3::new(0.0, 0.2, 0.0),
                radius_vel: 0.02,
                temperature: 1.0,
                temperature_speed: -0.004,
                scale_noise_coords: Vec3::new(0.2, 0.2, 0.2),
                scale_noise_result: 50.0,
            },
            SmokePart {
                offset: Vec3::new(32.0, -15.0, 32.0),
                radius: 2.0,
                velocity: Vec3::new(0.0, 0.2, 0.0),
                radius_vel: 0.02,
                temperature: 1.0,
                temperature_speed: -0.002,
                scale_noise_coords: Vec3::new(0.2, 0.2, 0.2),
                scale_noise_result: 60.0,
            },
            SmokePart {
                offset: Vec3::new(32.0, -30.0, 20.0),
                radius: 1.0,
                velocity: Vec3::new(0.0, 0.2, 0.0),
                radius_vel: 0.02,
                temperature: 1.0,
                temperature_speed: -0.002,
                scale_noise_coords: Vec3::new(0.2, 0.2, 0.2),
                scale_noise_result: 60.0,
            },
            SmokePart {
                offset: Vec3::new(20.0, -35.0, 32.0),
                radius: 1.0,
                velocity: Vec3::new(0.0, 0.2, 0.0),
                radius_vel: 0.02,
                temperature: 1.0,
                temperature_speed: -0.002,
                scale_noise_coords: Vec3::new(0.2, 0.2, 0.2),
                scale_noise_result: 60.0,
            },
        ];
        SmokeModel {
            size: UVec3::new(64, 128, 64),
            offset: Vec3::ZERO,
            rotation: Quat::from_axis_angle(Vec3::Y, 0.0),
            particles,
            noise: OpenSimplex::new(),
            cold_color: ColorRGBA::new(111, 123, 155, 255),
            hot_color: ColorRGBA::new(229, 88, 41, 255),
            very_hot_color: ColorRGBA::new(249, 195, 0, 255),
            ceiling_height: 42.0,
            ceiling_speed: -0.1,
        }
    }
}

impl HasCamera for SmokeScene {
    fn get_camera(&self) -> &Camera {
        &self.rendered.camera
    }
}

impl HasMutCamera for SmokeScene {
    fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.rendered.camera
    }
}

impl HasScene for SmokeScene {
    fn get_scene(&self) -> &Scene {
        &self.rendered
    }
}

impl Animatable for SmokeScene {
    fn animate(&mut self, frame: u32) {
        self.smoke.animate(frame);
        self.rendered.models = vec![self.smoke.generate()];
    }
}

impl HasBounding for SmokeScene {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.rendered.bounding()
    }
}
