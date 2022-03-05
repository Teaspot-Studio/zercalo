use glam::{UVec2, UVec3, Vec3};
use log::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use std::error::Error;

use zercalo_format::animation::{HasBounding, HasMutCamera, Renderable, RotationView};
use zercalo_format::procedure::smoke::{SmokeModel, SmokePart};
use zercalo_format::scene::{Camera, ColorRGB, ColorRGBA, Light, Model, Scene};
use zercalo_render::encode::save_frames;
use zercalo_render::render::render_frames;

const TILE_WIDTH: u32 = 64;
const TILE_HEIGHT: u32 = 128;
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 1024;
const FRAMES_COUNT: u32 = 512;

struct TestScene {
    smoke: SmokeModel,
    // cached
    rendered: Scene,
}

impl TestScene {
    fn new() -> RotationView<TestScene> {
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
        let model = SmokeModel {
            size: UVec3::new(64, 128, 64),
            particles,
            ceiling_height: 42.0,
            ceiling_speed: -0.1,
            ..SmokeModel::default()
        };
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
        // let test_model = Model::from_function(UVec3::new(16, 16, 16), |_| ColorRGBA::white());
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

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Zercalo voxel renderer", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    info!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_scale(7.0, 7.0)?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let scene = TestScene::new();

    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let tile_size = UVec2::new(TILE_WIDTH, TILE_HEIGHT);
    let mut frames = render_frames(
        &mut canvas,
        &texture_creator,
        FRAMES_COUNT,
        tile_size,
        scene,
    )?;
    save_frames(&mut canvas, &mut frames, tile_size, ".")?;

    let mut counter: u32 = 0;
    let mut frame = 0;
    'running: loop {
        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if counter % 2 == 0 {
            frame += 1;
            if frame >= frames.len() {
                frame = 0;
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let (sx, sy) = canvas.scale();
        canvas.copy(
            &frames[frame],
            None,
            Rect::new(
                ((WINDOW_WIDTH as f32 / (2.0 * sx)) as u32 - TILE_WIDTH / 2) as i32,
                ((WINDOW_HEIGHT as f32 / (2.0 * sy)) as u32 - TILE_HEIGHT / 2) as i32,
                TILE_WIDTH,
                TILE_HEIGHT,
            ),
        )?;
        canvas.present();

        counter += 1;
    }

    Ok(())
}
