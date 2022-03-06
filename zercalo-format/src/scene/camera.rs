use glam::{UVec2, Vec2, Vec3};

/// Defines distance between each pixel ray. Effectively scales image
pub const DEFAULT_PIXEL_SIZE: f32 = 0.7;
/// Defines maximum distance ray can travel before it considered as going to infininity.
pub const DEFAULT_RAY_MAX_DIST: f32 = 1024.0;
/// Default width of rendered tile
pub const DEFAULT_TILE_WIDTH: u32 = 64;
/// Default height of rendered tile
pub const DEFAULT_TILE_HEIGHT: u32 = 64;

#[derive(Clone, Debug)]
pub struct Camera {
    pub eye: Vec3,
    pub dir: Vec3,
    pub up: Vec3,
    pub pixel_size: f32,
    pub max_dist: f32,
    /// Size of resulted tile in pixels
    pub viewport: UVec2,
    /// How much the tile should be scaled
    pub view_scale: Vec2,
    /// Amount of frames to render
    pub max_frames: u32,
}

impl Default for Camera {
    fn default() -> Self {
        let eye = Vec3::new(32.0, 32.0, 32.0);
        Camera {
            eye,
            dir: -eye.normalize(),
            up: Vec3::new(0.0, 1.0, 0.0),
            pixel_size: DEFAULT_PIXEL_SIZE,
            max_dist: DEFAULT_RAY_MAX_DIST,
            viewport: UVec2::new(DEFAULT_TILE_WIDTH, DEFAULT_TILE_HEIGHT),
            view_scale: Vec2::new(7.0, 7.0),
            max_frames: 128,
        }
    }
}
