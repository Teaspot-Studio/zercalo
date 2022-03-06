pub mod scenes;

use log::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use std::error::Error;

use crate::scenes::*;
use zercalo_render::encode::save_frames;
use zercalo_render::render::render_frames;
use zercalo_format::animation::HasCamera;
use zercalo_format::scene::ColorRGBA;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 1024;

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
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // let scene = new_harvester_scene(ColorRGBA::player2())?;
    // let scene = SmokeScene::new();
    let scene = SandScene::new();

    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let cam = scene.get_camera();
    let tile_size = cam.viewport;
    canvas.set_scale(cam.view_scale.x, cam.view_scale.y)?;
    let mut frames = render_frames(
        &mut canvas,
        &texture_creator,
        cam.max_frames,
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

        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();
        let (sx, sy) = canvas.scale();
        canvas.copy(
            &frames[frame],
            None,
            Rect::new(
                ((WINDOW_WIDTH as f32 / (2.0 * sx)) as u32 - tile_size.x / 2) as i32,
                ((WINDOW_HEIGHT as f32 / (2.0 * sy)) as u32 - tile_size.y / 2) as i32,
                tile_size.x,
                tile_size.y,
            ),
        )?;
        canvas.present();

        counter += 1;
    }

    Ok(())
}
