use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

const TILE_SIZE: u32 = 64;
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 1024;

fn render_frame<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    let mut frame_texture = texture_creator
        .create_texture_target(None, TILE_SIZE, TILE_SIZE)
        .map_err(|e| e.to_string())?;

    {
        let textures = vec![(&mut frame_texture, "dummy")];
        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();

                for i in 0..TILE_SIZE {
                    for j in 0..TILE_SIZE {
                        if (i + j) % 4 == 0 {
                            texture_canvas.set_draw_color(Color::RGB(255, 255, 0));
                            texture_canvas
                                .draw_point(Point::new(i as i32, j as i32))
                                .expect("could not draw point");
                        }
                        if (i + j * 2) % 9 == 0 {
                            texture_canvas.set_draw_color(Color::RGB(200, 200, 0));
                            texture_canvas
                                .draw_point(Point::new(i as i32, j as i32))
                                .expect("could not draw point");
                        }
                    }
                }
            })
            .map_err(|e| e.to_string())?;
    }

    Ok(frame_texture)
}

fn main() -> Result<(), String> {
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

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let frame_texture = render_frame(&mut canvas, &texture_creator)?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame: u32 = 0;
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        let (sx, sy) = canvas.scale();
        canvas.copy(
            &frame_texture,
            None,
            Rect::new(
                ((WINDOW_WIDTH as f32 / (2.0 * sx)) as u32 - TILE_SIZE / 2) as i32,
                ((WINDOW_HEIGHT as f32 / (2.0 * sy)) as u32 - TILE_SIZE / 2) as i32,
                TILE_SIZE,
                TILE_SIZE,
            ),
        )?;
        canvas.present();
    }

    Ok(())
}
