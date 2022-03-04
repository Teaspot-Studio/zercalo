use super::scene::{Camera, Scene};
use glam::f32::Quat;
use glam::Vec3;

/// Trait that allows to wrap scene into more specific type and add animation state.
/// Renderer need to know only final scene state and how to step the scene between frames.
pub trait Renderable {
    /// Step scene for the next frame
    fn animate(&mut self, frame: u32);

    /// Get scene from the current state
    fn render(&self) -> &Scene;
}

/// We always can render static scene
impl Renderable for Scene {
    fn animate(&mut self, _frame: u32) {}

    fn render(&self) -> &Scene {
        self
    }
}

/// Trait that allows to access substate with camera
pub trait HasMutCamera {
    fn get_mut_camera(&'_ mut self) -> &'_ mut Camera;
}

impl HasMutCamera for Scene {
    fn get_mut_camera(&'_ mut self) -> &'_ mut Camera {
        &mut self.camera
    }
}

/// Trait that allows to access bounding volume of inner scene
pub trait HasBounding {
    fn get_bounding_volume(&self) -> (Vec3, Vec3);

    fn get_bounding_center(&self) -> Vec3 {
        let (minv, maxv) = self.get_bounding_volume();
        (maxv - minv) * 0.5
    }
}

impl HasBounding for Scene {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.bounding()
    }

    fn get_bounding_center(&self) -> Vec3 {
        self.center()
    }
}

pub struct RotationView<T> {
    pub scene: T,
}

impl<T: Renderable + HasMutCamera + HasBounding> Renderable for RotationView<T> {
    fn animate(&mut self, _frame: u32) {
        let quat = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI / 180.0);

        let target = self.scene.get_bounding_center();
        let cam = self.scene.get_mut_camera();
        cam.eye = target + quat.mul_vec3(cam.eye - target);
        cam.dir = (target - cam.eye).normalize();
    }

    fn render(&self) -> &Scene {
        self.scene.render()
    }
}
