use fast_voxel_traversal::raycast_3d::*;
use glam::f32::Quat;
use glam::{IVec3, Vec3};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const TILE_SIZE: u32 = 64;
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 1024;
const PIXEL_SIZE: f32 = 0.7;
const FRAMES_COUNT: u32 = 128;
const VOLUME_SIZE: u32 = 16;

fn save_png(str_path: &str, data: &[u8], width: u32, height: u32) {
    let path = Path::new(str_path);
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data).unwrap();
}

fn render_frames<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Vec<Texture<'a>>, String> {
    let mut frames = vec![];
    for _ in 0..FRAMES_COUNT {
        let frame = texture_creator
            .create_texture_target(None, TILE_SIZE, TILE_SIZE)
            .map_err(|e| e.to_string())?;
        frames.push(frame);
    }

    {
        let mut textures = vec![];
        let mut angle = 0.0;
        for (i, frame) in frames.iter_mut().enumerate() {
            textures.push((frame, (i, angle)));
            angle += std::f32::consts::PI / 60.0;
        }

        let volume = BoundingVolume3 { size: (16, 16, 16) };

        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, ctx| {
                let (i, angle) = *ctx;
                println!("Rendering frame {}/{}", i, FRAMES_COUNT);
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();

                for i in 0..TILE_SIZE {
                    for j in 0..TILE_SIZE {
                        let quat = Quat::from_axis_angle(Vec3::Y, angle);
                        let eye = quat.mul_vec3(Vec3::new(32.0, 32.0, 32.0));
                        let halfsize = VOLUME_SIZE as f32 * 0.5;
                        let camdir = (Vec3::new(halfsize, halfsize, halfsize) - eye).normalize();
                        let up = Vec3::new(0.0, 1.0, 0.0);
                        let right = camdir.cross(up);
                        let offset = up * ((j as f32 - 0.5 * TILE_SIZE as f32) * PIXEL_SIZE)
                            + right * ((i as f32 - 0.5 * TILE_SIZE as f32) * PIXEL_SIZE);

                        let ray = Ray3 {
                            origin: (eye + offset).into(),
                            direction: camdir.into(),
                            length: 100.0,
                        };

                        let light = Vec3::new(20.0, 25.0, 25.0);

                        for hit in volume.traverse_ray(ray).take(1) {
                            // The position of the voxel that was traversed. This will always be a voxel within the
                            // bounding volume.
                            let _position = hit.voxel;

                            // println!("{:?}", hit);
                            let inormal: IVec3 = hit.normal.unwrap_or((1, 0, 0)).into();
                            let normal: Vec3 = inormal.as_vec3();
                            let voxel: IVec3 = hit.voxel.into();
                            let tolight: Vec3 = (light - voxel.as_vec3()).normalize();
                            // println!("tolight: {:?}", tolight);
                            // println!("normal: {:?}", normal);
                            // println!("Light cos: {:?}", tolight.dot(normal));

                            let color = (255.0 / hit.distance * tolight.dot(normal)) as u8;
                            texture_canvas.set_draw_color(Color::RGB(color, color, color));
                            texture_canvas
                                .draw_point(Point::new(i as i32, (TILE_SIZE - j) as i32))
                                .expect("could not draw point");
                        }
                    }
                }
            })
            .map_err(|e| e.to_string())?;

        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, ctx| {
                let (i, _) = *ctx;
                println!("Saving frame {}/{}", i, FRAMES_COUNT);
                let pixels = texture_canvas
                    .read_pixels(None, PixelFormatEnum::ARGB8888)
                    .expect("Cannot read pixels from frame");
                save_png(
                    &format!("frame_{:0>4}.png", i),
                    &pixels,
                    TILE_SIZE,
                    TILE_SIZE,
                );
            })
            .map_err(|e| e.to_string())?;
    }

    Ok(frames)
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

    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let frames = render_frames(&mut canvas, &texture_creator)?;

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
