use glam::f32::Quat;
use glam::{UVec3, Vec3};
use std::ops::Bound;

use crate::animation::HasBounding;
use crate::scene::{ColorRGBA, Model};

#[derive(Clone, Debug)]
pub struct Particle {
    pub pos: Vec3,
    pub vel: Vec3,
    pub col: ColorRGBA,
    pub size: u8,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            col: ColorRGBA::empty(),
            size: 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParticlesModel {
    pub size: UVec3,
    pub offset: Vec3,
    pub rotation: Quat,
    pub particles: Vec<Particle>,
    pub gravity: Vec3,
}

impl Default for ParticlesModel {
    fn default() -> Self {
        ParticlesModel {
            size: UVec3::new(1, 1, 1),
            offset: Vec3::ZERO,
            rotation: Quat::from_axis_angle(Vec3::Y, 0.0),
            particles: vec![],
            gravity: Vec3::ZERO,
        }
    }
}

impl HasBounding for ParticlesModel {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        (
            self.offset,
            self.offset + self.rotation.mul_vec3(self.size.as_vec3()),
        )
    }
}

fn default_range<T>(v: (T, T)) -> (Bound<T>, Bound<T>) {
    (Bound::Included(v.0), Bound::Excluded(v.1))
}

fn random_f32(rng: &fastrand::Rng, start: f32, end: f32) -> f32 {
    rng.f32() * (end - start) + start
}

fn random_vec3(rng: &fastrand::Rng, range: (Vec3, Vec3)) -> Vec3 {
    let x = random_f32(rng, range.0.x, range.1.x);
    let y = random_f32(rng, range.0.y, range.1.y);
    let z = random_f32(rng, range.0.z, range.1.z);
    Vec3::new(x, y, z)
}

impl ParticlesModel {
    /// New empty model of particles
    pub fn new() -> Self {
        ParticlesModel::default()
    }

    /// Generate new random particles with given ranges for random starting position, velocity and size.
    pub fn new_random(
        rng: &fastrand::Rng,
        size: UVec3,
        amount_range: (u32, u32),
        vel_range: (Vec3, Vec3),
        pos_range: (Vec3, Vec3),
        size_range: (u8, u8),
        colors_pool: &[ColorRGBA],
    ) -> Self {
        let amount = rng.u32(default_range(amount_range));
        let mut particles = vec![Particle::default(); amount as usize];
        for p in particles.iter_mut() {
            *p = Particle {
                pos: random_vec3(rng, pos_range),
                vel: random_vec3(rng, vel_range),
                size: rng.u8(default_range(size_range)),
                col: colors_pool[rng.usize(default_range((0, colors_pool.len())))],
            };
        }

        ParticlesModel {
            size,
            particles,
            ..ParticlesModel::default()
        }
    }

    /// Render particles into voxel volume
    pub fn generate(&self) -> Model {
        let mut model = Model::new(self.size);

        for part in self.particles.iter() {
            let p1 = part.pos.max(Vec3::ZERO).as_uvec3();
            let p2 = (part.pos + part.size as f32)
                .min(model.size.as_vec3())
                .as_uvec3();

            for x in p1.x..p2.x {
                for y in p1.y..p2.y {
                    for z in p1.z..p2.z {
                        model.set_voxel(UVec3::new(x, y, z), part.col);
                    }
                }
            }
        }

        model.rotation = self.rotation;
        model.offset = self.offset;
        model
    }

    /// Step animation of particles
    pub fn animate(&mut self, _frame: u32) {
        for part in self.particles.iter_mut() {
            part.pos += part.vel;
            part.vel += self.gravity;
        }
    }
}
