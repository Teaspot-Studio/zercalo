use fast_voxel_traversal::raycast_3d::*;
use glam::f32::Quat;
use glam::{IVec3, UVec3, Vec3, Vec4};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use zercalo_format::scene::{ColorRGB, ColorRGBA, Light, Model, Scene};

const TILE_SIZE: u32 = 64;
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 1024;
const PIXEL_SIZE: f32 = 0.7;
const FRAMES_COUNT: u32 = 256;
const VOLUME_SIZE: u32 = 16;
const RAY_MAX_DIST: f32 = 100.0;

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
    let mut model1 = Model::from_function(UVec3::new(16, 16, 16), |_| ColorRGBA::white());
    model1.rotation = Quat::from_axis_angle(Vec3::X, std::f32::consts::PI / 6.0);
    model1.offset = Vec3::new(5.0, 0.0, 0.0);
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
    Scene {
        models: vec![model1, model2],
        lights: vec![light1, light2],
        ..Scene::default()
    }
}

fn animate_scene(scene: &mut Scene, _frame: u32) {
    let quat = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI / 180.0);
    let halfsize = VOLUME_SIZE as f32 * 0.5;

    let target = Vec3::new(halfsize, halfsize, halfsize);
    scene.camera.eye = target + quat.mul_vec3(scene.camera.eye - target);
    scene.camera.dir = (target - scene.camera.eye).normalize();
}

fn blend_colors(src: Vec4, dst: Vec4) -> Vec4 {
    let dist_factor = dst.w * (1.0 - src.w);
    let mut res = src;
    res *= src.w;
    res.x += dst.x * dist_factor;
    res.y += dst.y * dist_factor;
    res.z += dst.z * dist_factor;
    res.w += dist_factor;
    res
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
                        // Total accumulated color for model
                        let mut total_color = Vec4::new(0.0, 0.0, 0.0, 0.0);
                        // The last distance ray traveled until full stop. It is used to cull other models.
                        let mut total_dist = RAY_MAX_DIST;
                        for model in scene.models.iter() {
                            let rot_quat = model.rotation.inverse();
                            let eye = rot_quat.mul_vec3(scene.camera.eye);
                            let up = rot_quat.mul_vec3(scene.camera.up);
                            let dir = rot_quat.mul_vec3(scene.camera.dir);

                            let right = dir.cross(up);
                            let offset = up * ((j as f32 - 0.5 * TILE_SIZE as f32) * PIXEL_SIZE)
                                + right * ((i as f32 - 0.5 * TILE_SIZE as f32) * PIXEL_SIZE);

                            let ray_origin = eye - model.offset + offset;
                            let ray = Ray3 {
                                origin: ray_origin.into(),
                                direction: dir.into(),
                                length: RAY_MAX_DIST,
                            };

                            let volume = BoundingVolume3 {
                                size: (
                                    model.size.x as i32,
                                    model.size.y as i32,
                                    model.size.z as i32,
                                ),
                            };

                            let mut model_color = Vec4::new(0.0, 0.0, 0.0, 0.0);
                            let mut model_dist = RAY_MAX_DIST;
                            'rayloop: for hit in volume.traverse_ray(ray) {
                                let inormal: IVec3 = hit.normal.unwrap_or((1, 0, 0)).into();
                                let normal: Vec3 = inormal.as_vec3();
                                let voxel: IVec3 = hit.voxel.into();
                                let diffuse: Vec4 = model[voxel.as_uvec3()].as_vec4();

                                let mut light_component = Vec3::new(0.0, 0.0, 0.0);
                                for light in scene.lights.iter() {
                                    let tolight: Vec3 =
                                        (light.position - voxel.as_vec3()).normalize();
                                    let new_component = diffuse.truncate()
                                        * light.color.as_vec3()
                                        * tolight.dot(normal);
                                    light_component += new_component.max(Vec3::new(0.0, 0.0, 0.0));
                                }

                                model_color = blend_colors(model_color, (light_component, diffuse.w).into());
                                model_dist = (ray_origin - voxel.as_vec3()).length();
                                if model_color.w >= 1.0 {
                                    break 'rayloop;
                                }
                            }
                            // println!("model_dist = {}, total_dist = {}", model_dist, total_dist);

                            if model_dist <= total_dist {
                                total_color = blend_colors(model_color, total_color);
                                total_dist = model_dist;
                            } else {
                                total_color = blend_colors(total_color, model_color);
                            }
                        }

                        texture_canvas.set_draw_color(Color::RGB(
                            (total_color.x * 255.0) as u8,
                            (total_color.y * 255.0) as u8,
                            (total_color.z * 255.0) as u8,
                        ));
                        texture_canvas
                            .draw_point(Point::new(i as i32, (TILE_SIZE - j) as i32))
                            .expect("could not draw point");
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
