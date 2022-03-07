use glam::{UVec2, Vec2, Vec3};
use maplit::hashmap;
use zercalo_format::animation::{Animatable, RotationView, Switcher};
use zercalo_format::color::{ColorRGB, ColorRGBA};
use zercalo_format::import::vox::{from_vox_file, VoxImportError};
use zercalo_format::scene::{
    Camera, HasBounding, HasCamera, HasMutCamera, HasScene, Light, Model, Scene,
};

pub struct HarvesterScene {
    tracker_right: Switcher<Model>,
    tracker_left: Switcher<Model>,
    body: Model,
    collector: Model,
    /// Scene is cached to store voxels for renderer
    rendered: Scene,
}

pub fn new_harvester_scene(
    player_color: ColorRGBA,
) -> Result<RotationView<HarvesterScene>, VoxImportError> {
    let mut body = from_vox_file("./assets/models/harvester_body.vox")?[0].clone();
    body.replace_colors = hashmap! {
        ColorRGBA::new(183, 183, 183, 255) => player_color,
        ColorRGBA::new(23, 84, 131, 255) => ColorRGBA::new(23, 84, 131, 100),
    };
    body.offset = Vec3::new(4., 0., 0.);

    let tracker_right = new_tracker()?;
    let mut tracker_left = tracker_right.clone();
    for m in tracker_left.variants.iter_mut() {
        m.offset = Vec3::new(16.0, 0., 0.);
    }

    let mut collector = from_vox_file("./assets/models/harvester_collector.vox")?[0].clone();
    collector.offset = Vec3::new(0.0, 0.0, 32.0);

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
    let ext_scene = HarvesterScene {
        tracker_right,
        tracker_left,
        body,
        collector,
        rendered: scene,
    };
    Ok(RotationView {
        scene: ext_scene,
        target_y: Some(8.0),
        rotation_speed: std::f32::consts::PI / 180.0,
    })
}

fn new_tracker() -> Result<Switcher<Model>, VoxImportError> {
    let paths = vec![
        (5, "./assets/models/harvester_track_01.vox"),
        (5, "./assets/models/harvester_track_02.vox"),
        (5, "./assets/models/harvester_track_03.vox"),
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

impl HasCamera for HarvesterScene {
    fn get_camera(&self) -> &Camera {
        &self.rendered.camera
    }
}

impl HasMutCamera for HarvesterScene {
    fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.rendered.camera
    }
}

impl HasScene for HarvesterScene {
    fn get_scene(&self) -> &Scene {
        &self.rendered
    }
}

impl Animatable for HarvesterScene {
    fn animate(&mut self, frame: u32) {
        self.tracker_right.animate(frame);
        self.tracker_left.animate(frame);
        self.rendered.models = vec![
            self.tracker_right.current().clone(),
            self.tracker_left.current().clone(),
            self.body.clone(),
            self.collector.clone(),
        ];
    }
}

impl HasBounding for HarvesterScene {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.rendered.bounding()
    }
}
