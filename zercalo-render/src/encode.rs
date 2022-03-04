use glam::UVec2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("Failed to write down a file: {0}")]
    File(#[from] std::io::Error),
    #[error("Failed to encode APNG: {0}")]
    Apng(String),
    #[error("Failed to encode PNG: {0}")]
    Png(#[from] png::EncodingError),
    #[error("Failed to render to texture: {0}")]
    Render(#[from] sdl2::render::TargetRenderError),
}

fn save_png(str_path: &str, data: &[u8], width: u32, height: u32) -> Result<(), EncodeError> {
    let path = Path::new(str_path);
    let file = File::create(path)?;
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(data)?;
    Ok(())
}

fn save_apng<'a, V: IntoIterator<Item = &'a [u8]>>(
    str_path: &str,
    data: V,
    width: u32,
    height: u32,
) -> Result<(), EncodeError> {
    let path = Path::new(str_path);
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);
    let images: Vec<apng::PNGImage> = data
        .into_iter()
        .map(|datum| apng::PNGImage {
            width,
            height,
            data: datum.to_vec(),
            color_type: png::ColorType::RGBA,
            bit_depth: png::BitDepth::Eight,
        })
        .collect();

    let config = apng::Config {
        width,
        height,
        num_frames: images.len() as u32,
        num_plays: 1,
        color: png::ColorType::RGBA,
        depth: png::BitDepth::Eight,
        filter: png::FilterType::NoFilter,
    };
    let frame_cfg = apng::Frame {
        delay_den: Some(24),
        ..apng::Frame::default()
    };
    let mut encoder =
        apng::Encoder::new(&mut w, config).map_err(|e| EncodeError::Apng(e.to_string()))?;
    encoder
        .encode_all(images, Some(&frame_cfg))
        .map_err(|e| EncodeError::Apng(e.to_string()))?;
    encoder
        .finish_encode()
        .map_err(|e| EncodeError::Apng(e.to_string()))?;

    Ok(())
}

pub fn save_frames<'a>(
    canvas: &mut Canvas<Window>,
    frames: &mut [Texture<'a>],
    tile_size: UVec2,
    directory: &str,
) -> Result<(), EncodeError> {
    // let frames_count = frames.len();
    let mut textures = vec![];
    for (i, frame) in frames.iter_mut().enumerate() {
        textures.push((frame, i));
    }

    let mut frames_data = vec![];
    canvas.with_multiple_texture_canvas(textures.iter(), |texture_canvas, _| {
        // println!("Saving frame {}/{}", i, frames_count);
        let pixels = texture_canvas
            .read_pixels(None, PixelFormatEnum::ABGR8888)
            .expect("Cannot read pixels from frame");
        frames_data.push(pixels);
    })?;

    fs::create_dir_all(format!("{}/frames/diffuse", directory))?;
    for (i, pixels) in frames_data.iter().enumerate() {
        save_png(
            &format!("{}/frames/diffuse/frame_{:0>4}.png", directory, i),
            pixels,
            tile_size.x,
            tile_size.y,
        )?;
    }
    save_apng(
        &format!("{}/diffuse.png", directory),
        frames_data.iter().map(|v| &v[..]),
        tile_size.x,
        tile_size.y,
    )?;

    Ok(())
}
