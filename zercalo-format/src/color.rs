use glam::{Vec3, Vec4};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ColorRGB { r, g, b }
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
        )
    }

    pub fn white() -> Self {
        ColorRGB::new(255, 255, 255)
    }

    pub fn black() -> Self {
        ColorRGB::new(0, 0, 0)
    }
}

impl Default for ColorRGB {
    fn default() -> Self {
        ColorRGB::new(0, 0, 0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        ColorRGBA { r, g, b, a }
    }

    pub fn as_vec4(&self) -> Vec4 {
        Vec4::new(
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
            (self.a as f32) / 255.0,
        )
    }

    pub fn empty() -> Self {
        ColorRGBA::new(0, 0, 0, 0)
    }

    pub fn is_empty(&self) -> bool {
        self.a == 0
    }

    pub fn white() -> Self {
        ColorRGBA::new(255, 255, 255, 255)
    }

    pub fn black() -> Self {
        ColorRGBA::new(0, 0, 0, 255)
    }

    pub fn with_alpha(&self, a: u8) -> Self {
        ColorRGBA {
            r: self.r,
            g: self.g,
            b: self.b,
            a,
        }
    }

    /// Convert color to Vec4 where each component are in range 0 .. 1.0 and
    /// premultiplied by alpha value to correctly process alpha blending.
    ///
    /// In that representation RGB components represents how much light of
    /// each channel is captured.
    pub fn as_premultipied(&self) -> Vec4 {
        let x = (self.r as f32) / 255.0;
        let y = (self.g as f32) / 255.0;
        let z = (self.b as f32) / 255.0;
        let w = (self.a as f32) / 255.0;
        Vec4::new(x * w, y * w, z * w, w)
    }

    /// Convert color from Vec4 where each component are in range 0 .. 1.0 and
    /// premultiplied by alpha value to correctly process alpha blending.
    ///
    /// In that representation RGB components represents how much light of
    /// each channel is captured.
    pub fn from_premultiplied(v: &Vec4) -> Self {
        let r = (v.x / v.w * 255.0) as u8;
        let g = (v.y / v.w * 255.0) as u8;
        let b = (v.z / v.w * 255.0) as u8;
        let a = (v.w * 255.0) as u8;
        ColorRGBA::new(r, g, b, a)
    }

    pub fn player1() -> Self {
        ColorRGBA::new(240, 0, 0, 255)
    }

    pub fn player2() -> Self {
        ColorRGBA::new(0, 0, 240, 255)
    }
}

impl Default for ColorRGBA {
    fn default() -> Self {
        ColorRGBA::new(0, 0, 0, 255)
    }
}
