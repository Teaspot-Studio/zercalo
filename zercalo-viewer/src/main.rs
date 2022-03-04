use glam::f32::Quat;
use glam::{UVec3, Vec3, UVec2};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use std::error::Error;

use zercalo_format::scene::{ColorRGB, ColorRGBA, Light, Model, Scene};
use zercalo_format::animation::RotationView;
use zercalo_render::render::render_frames;
use zercalo_render::encode::save_frames;

const TILE_SIZE: u32 = 64;
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 1024;
const FRAMES_COUNT: u32 = 256;

fn test_scene() -> RotationView {
    let mut model1 = Model::from_function(UVec3::new(16, 16, 16), |_| ColorRGBA::new(200, 100, 0, 255));
    model1.rotation = Quat::from_axis_angle(Vec3::X, std::f32::consts::PI / 6.0);
    model1.offset = Vec3::new(5.0, 1.0, 0.0);
    let mut model2 = Model::from_function(UVec3::new(16, 16, 16), |_| ColorRGBA::white());
    model2.rotation = Quat::from_axis_angle(Vec3::X, -std::f32::consts::PI / 6.0);
    let light1 = Light {
        position: Vec3::new(25.0, 20.0, 20.0),
        color: ColorRGB::new(255, 230, 230),
    };
    let light2 = Light {
        position: Vec3::new(-25.0, 20.0, -20.0),
        color: ColorRGB::new(210, 210, 255),
    };
    let scene = Scene {
        models: vec![model1, model2],
        lights: vec![light1, light2],
        ..Scene::default()
    };
    RotationView {
        scene
    }
}

fn main() -> Result<(), Box<dyn Error>> {
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

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_scale(7.0, 7.0)?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let scene = test_scene();

    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let tile_size = UVec2::new(TILE_SIZE, TILE_SIZE);
    let mut frames = render_frames(&mut canvas, &texture_creator, FRAMES_COUNT, tile_size, scene)?;
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
                ((WINDOW_WIDTH as f32 / (2.0 * sx)) as u32 - TILE_SIZE / 2) as i32,
                ((WINDOW_HEIGHT as f32 / (2.0 * sy)) as u32 - TILE_SIZE / 2) as i32,
                TILE_SIZE,
                TILE_SIZE,
            ),
        )?;
        canvas.present();

        counter += 1;
    }

    Ok(())
}
