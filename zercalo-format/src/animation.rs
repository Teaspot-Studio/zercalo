use super::scene::Scene;
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
    fn animate(&mut self, _frame: u32) {

    }

    fn render(&self) -> &Scene {
        self
    }
}

pub struct RotationView {
    pub scene: Scene
}

impl Renderable for RotationView {
    fn animate(&mut self, _frame: u32) {
        let quat = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI / 180.0);

        let target = self.scene.center();
        println!("{:?}", target);
        self.scene.camera.eye = target + quat.mul_vec3(self.scene.camera.eye - target);
        self.scene.camera.dir = (target - self.scene.camera.eye).normalize();
    }

    fn render(&self) -> &Scene {
        &self.scene
    }
}
