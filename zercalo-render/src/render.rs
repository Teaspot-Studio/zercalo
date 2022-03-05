use fast_voxel_traversal::raycast_3d::*;
use glam::{IVec3, UVec2, Vec3, Vec4};
use log::*;
use rayon::prelude::*;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use thiserror::Error;

use zercalo_format::animation::Renderable;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("Failed to create textures: {0}")]
    Texture(#[from] sdl2::render::TextureValueError),
    #[error("Failed to render to texture: {0}")]
    Render(#[from] sdl2::render::TargetRenderError),
}

fn blend_colors(src: Vec4, dst: Vec4) -> Vec4 {
    let dist_factor = dst.w * (1.0 - src.w);
    let mut res = src;
    res.x = src.x * src.w + dst.x * dist_factor;
    res.y = src.y * src.w + dst.y * dist_factor;
    res.z = src.z * src.w + dst.z * dist_factor;
    res.w += dist_factor;
    res
}

pub fn render_frames<'a, R: Renderable>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    frames_count: u32,
    tile_size: UVec2,
    mut context: R,
) -> Result<Vec<Texture<'a>>, RenderError> {
    let mut frames = vec![];
    for _ in 0..frames_count {
        let frame = texture_creator.create_texture_target(
            Some(PixelFormatEnum::RGBA8888),
            tile_size.x,
            tile_size.y,
        )?;
        frames.push(frame);
    }

    {
        let mut textures = vec![];
        for (i, frame) in frames.iter_mut().enumerate() {
            textures.push((frame, i));
        }

        canvas.with_multiple_texture_canvas(textures.iter(), |texture_canvas, frame| {
            info!("Rendering frame {}/{}", frame, frames_count);
            texture_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
            texture_canvas.clear();
            context.animate(*frame as u32);
            let scene = context.render();

            // First render columns in parallel
            let mut columns = vec![];
            (0..tile_size.x)
                .into_par_iter()
                .map(|i| {
                    let mut column = vec![];
                    (0..tile_size.y)
                        .into_par_iter()
                        .map(|j| {
                            // Total accumulated color for model
                            let mut total_color = Vec4::new(0.0, 0.0, 0.0, 0.0);
                            // The last distance ray traveled until full stop. It is used to cull other models.
                            let mut total_dist = scene.camera.max_dist;
                            for model in scene.models.iter() {
                                let rot_quat = model.rotation.inverse();
                                let eye = rot_quat.mul_vec3(scene.camera.eye);
                                let up = rot_quat.mul_vec3(scene.camera.up);
                                let dir = rot_quat.mul_vec3(scene.camera.dir);

                                let right = dir.cross(up);
                                let offset = up
                                    * ((j as f32 - 0.5 * tile_size.y as f32)
                                        * scene.camera.pixel_size)
                                    + right
                                        * ((i as f32 - 0.5 * tile_size.x as f32)
                                            * scene.camera.pixel_size);

                                let ray_origin = eye - model.offset + offset;
                                let ray = Ray3 {
                                    origin: ray_origin.into(),
                                    direction: dir.into(),
                                    length: scene.camera.max_dist,
                                };

                                let volume = BoundingVolume3 {
                                    size: (
                                        model.size.x as i32,
                                        model.size.y as i32,
                                        model.size.z as i32,
                                    ),
                                };

                                let mut model_color = Vec4::ZERO;
                                let mut model_dist = scene.camera.max_dist;
                                'rayloop: for hit in volume.traverse_ray(ray) {
                                    let inormal: IVec3 = hit.normal.unwrap_or((1, 0, 0)).into();
                                    let normal: Vec3 = inormal.as_vec3();
                                    let voxel: IVec3 = hit.voxel.into();
                                    let diffuse_orig = model[voxel.as_uvec3()];
                                    let diffuse = model
                                        .replace_colors
                                        .get(&diffuse_orig)
                                        .unwrap_or(&diffuse_orig)
                                        .as_vec4();

                                    let mut light_component = Vec3::new(0.0, 0.0, 0.0);
                                    for light in scene.lights.iter() {
                                        let tolight: Vec3 = (rot_quat.mul_vec3(light.position)
                                            - voxel.as_vec3())
                                        .normalize();
                                        let new_component = diffuse.truncate()
                                            * light.color.as_vec3()
                                            * tolight.dot(normal);
                                        light_component +=
                                            new_component.max(Vec3::new(0.0, 0.0, 0.0));
                                    }
                                    let ambient_component =
                                        diffuse.truncate() * scene.ambient.as_vec3();
                                    model_color = blend_colors(
                                        model_color,
                                        (ambient_component + light_component, diffuse.w).into(),
                                    );
                                    model_dist = (ray_origin - voxel.as_vec3()).length();
                                    if model_color.w >= 1.0 {
                                        break 'rayloop;
                                    }
                                }

                                if model_dist <= total_dist {
                                    total_color = blend_colors(model_color, total_color);
                                    total_dist = model_dist;
                                } else {
                                    total_color = blend_colors(total_color, model_color);
                                }
                            }
                            total_color
                        })
                        .collect_into_vec(&mut column);
                    column
                })
                .collect_into_vec(&mut columns);

            // Writing down colors to texture
            for (i, column) in columns.iter().enumerate() {
                for (j, total_color) in column.iter().enumerate() {
                    texture_canvas.set_draw_color(Color::RGBA(
                        (total_color.x * 255.0) as u8,
                        (total_color.y * 255.0) as u8,
                        (total_color.z * 255.0) as u8,
                        (total_color.w * 255.0) as u8,
                    ));
                    texture_canvas
                        .draw_point(Point::new(i as i32, (tile_size.y - j as u32) as i32))
                        .expect("could not draw point");
                }
            }
        })?;
    }

    Ok(frames)
}
