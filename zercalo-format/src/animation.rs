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

/// Trait for complex scenes that has embedded scene inside
pub trait HasScene {
    fn get_scene(&self) -> &Scene;
}

impl HasScene for Scene {
    fn get_scene(&self) -> &Scene {
        self
    }
}

/// Trait for complex scenes that has embedded mutable scene inside
pub trait HasMutScene {
    fn get_scene_mut(&mut self) -> &mut Scene;
}

impl HasMutScene for Scene {
    fn get_scene_mut(&mut self) -> &mut Scene {
        self
    }
}

/// Trait that allows to access substate with camera
pub trait HasCamera {
    fn get_camera(& self) -> &Camera;
}

impl HasCamera for Scene {
    fn get_camera(& self) -> &Camera {
        &self.camera
    }
}

/// Trait that allows to access substate with camera
pub trait HasMutCamera {
    fn get_mut_camera(&mut self) -> &mut Camera;
}

impl HasMutCamera for Scene {
    fn get_mut_camera(&mut self) -> &mut Camera {
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
