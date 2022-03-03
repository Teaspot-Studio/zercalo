use fast_voxel_traversal::raycast_3d::*;
use glam::f32::Quat;
use glam::{IVec3, UVec3, Vec3};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use zercalo_format::scene::{Model, Scene, Light, ColorRGB};

const TILE_SIZE: u32 = 64;
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 1024;
const PIXEL_SIZE: f32 = 0.7;
const FRAMES_COUNT: u32 = 256;
const VOLUME_SIZE: u32 = 16;

fn save_png(str_path: &str, data: &[u8], width: u32, height: u32) {
    let path = Path::new(str_path);
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
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

fn test_scene() -> Scene {
    let model = Model {
        size: UVec3::new(16, 16, 16),
    };
    let light1 = Light {
        position: Vec3::new(25.0, 20.0, 20.0),
        color: ColorRGB::new(255, 230, 230),
    };
    let light2 = Light {
        position: Vec3::new(-25.0, 20.0, -20.0),
        color: ColorRGB::new(210, 210, 255),
    };
    Scene {
        models: vec![model],
        lights: vec![light1, light2],
        ..Scene::default()
    }
}

fn animate_scene(scene: &mut Scene, _frame: u32) {
    let quat = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI / 180.0);
    let halfsize = VOLUME_SIZE as f32 * 0.5;

    scene.camera.eye = quat.mul_vec3(scene.camera.eye);
    scene.camera.dir = (Vec3::new(halfsize, halfsize, halfsize) - scene.camera.eye).normalize();
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
        for (i, frame) in frames.iter_mut().enumerate() {
            textures.push((frame, i));
        }

        let mut scene = test_scene();

        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, frame| {
                println!("Rendering frame {}/{}", frame, FRAMES_COUNT);
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                animate_scene(&mut scene, *frame as u32);

                for i in 0..TILE_SIZE {
                    for j in 0..TILE_SIZE {

                        let right = scene.camera.dir.cross(scene.camera.up);
                        let offset = scene.camera.up
                            * ((j as f32 - 0.5 * TILE_SIZE as f32) * PIXEL_SIZE)
                            + right * ((i as f32 - 0.5 * TILE_SIZE as f32) * PIXEL_SIZE);

                        let ray = Ray3 {
                            origin: (scene.camera.eye + offset).into(),
                            direction: scene.camera.dir.into(),
                            length: 100.0,
                        };

                        for model in scene.models.iter() {
                            let volume = BoundingVolume3 {
                                size: (
                                    model.size.x as i32,
                                    model.size.y as i32,
                                    model.size.z as i32,
                                ),
                            };

                            for hit in volume.traverse_ray(ray).take(1) {
                                // println!("{:?}", hit);
                                let inormal: IVec3 = hit.normal.unwrap_or((1, 0, 0)).into();
                                let normal: Vec3 = inormal.as_vec3();
                                let voxel: IVec3 = hit.voxel.into();

                                let mut light_component = Vec3::new(0.0, 0.0, 0.0);
                                for light in scene.lights.iter() {
                                    let tolight: Vec3 =
                                        (light.position - voxel.as_vec3()).normalize();
                                    let new_component = light.color.as_vec3() * tolight.dot(normal);
                                    light_component += new_component.max(Vec3::new(0.0, 0.0, 0.0));
                                }
                                println!("{:?}", light_component);

                                texture_canvas.set_draw_color(Color::RGB(
                                    (light_component.x * 255.0) as u8,
                                    (light_component.y * 255.0) as u8,
                                    (light_component.z * 255.0) as u8,
                                ));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, (TILE_SIZE - j) as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                }
            })
            .map_err(|e| e.to_string())?;

        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, i| {
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
