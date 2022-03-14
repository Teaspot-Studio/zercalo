use glam::{UVec2, Vec2, Vec3};
use log::*;
use zercalo_format::animation::{Animatable, RotationView, Stepper, Switcher};
use zercalo_format::color::ColorRGB;
use zercalo_format::import::vox::{from_vox_file, VoxImportError};
use zercalo_format::scene::{
    Camera, HasBounding, HasCamera, HasMutCamera, HasScene, Light, Model, Scene,
};

pub struct SandWormScene {
    body: Switcher<Stepper<Switcher<Model>>>,
    /// Scene is cached to store voxels for renderer
    rendered: Scene,
}

pub fn new_sandworm_scene() -> Result<RotationView<SandWormScene>, VoxImportError> {
    let zstep = 0.4;
    let zstart = -12.0;
    let ascending = make_body_ascending()?;
    let ascending_cycle = ascending.cycle_len();
    let stage0 = Stepper::new(ascending, move |m, frames| {
        let i = frames % ascending_cycle;
        debug!(
            "Stage0 stepper frame {}, internal frame {}, active model {}",
            frames, i, m.active
        );
        m.current_mut().offset = Vec3::new(0., i as f32 * zstep + zstart, 0.);
    });

    let descending = make_body_descending()?;
    let descending_cycle = descending.cycle_len();
    let zend = ascending_cycle as f32 * zstep + zstart;
    let stage1 = Stepper::new(descending, move |m, frames| {
        let i = (frames - ascending_cycle) % descending_cycle;
        debug!(
            "Stage1 stepper frame {}, internal frame {}, active model {}",
            frames, i, m.active
        );
        m.current_mut().offset = Vec3::new(0., zend - i as f32 * zstep, 0.);
    });

    let body = Switcher::new(vec![(ascending_cycle, stage0), (descending_cycle, stage1)]);
    let eye = Vec3::new(128., 128., 128.);
    let scene = Scene {
        camera: Camera {
            eye,
            dir: -eye.normalize(),
            pixel_size: 0.5,
            viewport: UVec2::new(128, 128),
            view_scale: Vec2::new(7.0, 7.0),
            max_frames: 512,
            ..Camera::default()
        },
        lights: vec![Light {
            position: Vec3::new(128.0, 150.0, 75.0),
            color: ColorRGB::white(),
        }],
        ..Scene::default()
    };
    let ext_scene = SandWormScene {
        body,
        rendered: scene,
    };
    Ok(RotationView {
        scene: ext_scene,
        target_y: Some(10.0),
        rotation_speed: std::f32::consts::PI / 180.0,
    })
}

fn make_body_ascending() -> Result<Switcher<Model>, VoxImportError> {
    let paths = vec![
        (5, "./assets/models/sandworm/worm_01.vox"),
        (5, "./assets/models/sandworm/worm_02.vox"),
        (5, "./assets/models/sandworm/worm_03.vox"),
        (5, "./assets/models/sandworm/worm_04.vox"),
        (5, "./assets/models/sandworm/worm_05.vox"),
        (5, "./assets/models/sandworm/worm_06.vox"),
    ];
    let models: Result<Vec<(u32, Model)>, VoxImportError> = paths
        .into_iter()
        .map(|(dur, path)| {
            from_vox_file(path)
                .map(|vs| (dur, vs.into_iter().next().expect("Zero models in vox file")))
        })
        .collect();
    Ok(Switcher::new(models?))
}

fn make_body_descending() -> Result<Switcher<Model>, VoxImportError> {
    let path = "./assets/models/sandworm/worm_06.vox";
    let model = from_vox_file(path)?
        .into_iter()
        .next()
        .expect("Zero models in vox file");
    let models = (0..6).into_iter().map(|_| (5, model.clone())).collect();
    Ok(Switcher::new(models))
}

impl HasCamera for SandWormScene {
    fn get_camera(&self) -> &Camera {
        &self.rendered.camera
    }
}

impl HasMutCamera for SandWormScene {
    fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.rendered.camera
    }
}

impl HasScene for SandWormScene {
    fn get_scene(&self) -> &Scene {
        &self.rendered
    }
}

impl Animatable for SandWormScene {
    fn animate(&mut self, frame: u32) {
        self.body.animate(frame);
        self.rendered.models = vec![self.body.current().value.current().clone()];
    }
}

impl HasBounding for SandWormScene {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.rendered.bounding()
    }
}
