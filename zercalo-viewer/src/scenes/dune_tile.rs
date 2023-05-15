use glam::{UVec2, UVec3, Vec2, Vec3};
use zercalo_format::animation::{Animatable, RotationView};
use zercalo_format::color::{ColorRGB, ColorRGBA};
use zercalo_format::scene::{
    Camera, HasBounding, HasCamera, HasMutCamera, HasScene, Light, Model, Scene,
};

pub struct DuneTile {
    /// Scene is cached to store voxels for renderer
    rendered: Scene,
}

/// Select value depending on the assigned weight
fn weighted<'a, T>(rng: &mut fastrand::Rng, values: &'a [(T, f32)]) -> &'a T {
    assert!(!values.is_empty(), "Empty weighted list");
    let total_weight: f32 = values.iter().map(|(_, w)| w).sum();
    let selected = rng.f32() * total_weight;
    let mut accum = 0.0;
    for (v, w) in values {
        if selected < accum + w {
            return v;
        }
        accum += w;
    }
    &values[0].0
}

impl DuneTile {
    pub fn new() -> RotationView<Self> {
        let mut rng = fastrand::Rng::with_seed(42);
        // let colors = vec![
        //     ColorRGBA::new(242, 183, 106, 100),
        //     ColorRGBA::new(232, 198, 150, 100),
        //     ColorRGBA::new(255, 145, 56, 100),
        // ];
        let color1 = ColorRGBA::new(242, 183, 106, 255);
        let color2 = ColorRGBA::new(232, 198, 150, 255);
        let color3 = ColorRGBA::new(225, 145, 56, 255);
        let colors = vec![(color1, 0.8), (color2, 0.1), (color3, 0.1)];
        let size = UVec3::new(64, 64, 64);
        let mut model = Model::new(size);
        for i in 0..size.x {
            for j in 0..size.z {
                let height =
                    (10.0 + 0.05 * f32::sin(7.0 * i as f32 / size.x as f32) * size.y as f32).round().max(1.0) as u32;
                for z in 0 .. height {
                    model.set_voxel(
                        UVec3::new(i, z, j),
                        weighted(&mut rng, &colors).clone(),
                    );
                }
            }
        }

        let eye = Vec3::new(256., 256., 256.);
        let scene = Scene {
            camera: Camera {
                eye,
                dir: -eye.normalize(),
                pixel_size: 1.0,
                viewport: UVec2::new(128, 128),
                view_scale: Vec2::new(4.0, 4.0),
                max_frames: 1,
                ..Camera::default()
            },
            lights: vec![Light {
                position: Vec3::new(128.0, 150.0, 75.0),
                color: ColorRGB::white(),
            }],
            models: vec![model],
            ..Scene::default()
        };
        let mut sand_scene = DuneTile { rendered: scene };
        sand_scene.animate(0);
        RotationView {
            scene: sand_scene,
            target_y: Some(0.0),
            rotation_speed: 0.0,
        } // std::f32::consts::PI / 180.0
    }
}

impl HasCamera for DuneTile {
    fn get_camera(&self) -> &Camera {
        &self.rendered.camera
    }
}

impl HasMutCamera for DuneTile {
    fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.rendered.camera
    }
}

impl HasScene for DuneTile {
    fn get_scene(&self) -> &Scene {
        &self.rendered
    }
}

impl Animatable for DuneTile {
    fn animate(&mut self, frame: u32) {}
}

impl HasBounding for DuneTile {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.rendered.bounding()
    }
}
