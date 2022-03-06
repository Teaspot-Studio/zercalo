use super::renderable::Renderable;
use crate::scene::{Camera, HasBounding, HasCamera, HasMutCamera, HasMutScene, HasScene, Scene};
use glam::f32::Quat;
use glam::Vec3;

pub struct RotationView<T> {
    pub scene: T,
    pub target_y: Option<f32>,
    pub rotation_speed: f32,
}

impl<T: HasCamera> HasCamera for RotationView<T> {
    fn get_camera(&self) -> &Camera {
        self.scene.get_camera()
    }
}

impl<T: HasMutCamera> HasMutCamera for RotationView<T> {
    fn get_mut_camera(&mut self) -> &mut Camera {
        self.scene.get_mut_camera()
    }
}

impl<T: HasBounding> HasBounding for RotationView<T> {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.scene.get_bounding_volume()
    }
}

impl<T: HasScene> HasScene for RotationView<T> {
    fn get_scene(&self) -> &Scene {
        self.scene.get_scene()
    }
}

impl<T: HasMutScene> HasMutScene for RotationView<T> {
    fn get_scene_mut(&mut self) -> &mut Scene {
        self.scene.get_scene_mut()
    }
}

impl<T: Renderable + HasMutCamera + HasBounding> Renderable for RotationView<T> {
    fn animate(&mut self, frame: u32) {
        self.scene.animate(frame);

        let quat = Quat::from_axis_angle(Vec3::Y, self.rotation_speed);

        let mut target = self.scene.get_bounding_center();
        if let Some(y) = self.target_y {
            target.y = y;
        }
        let cam = self.scene.get_mut_camera();
        cam.eye = target + quat.mul_vec3(cam.eye - target);
        cam.dir = (target - cam.eye).normalize();
    }

    fn render(&self) -> &Scene {
        self.scene.render()
    }
}
