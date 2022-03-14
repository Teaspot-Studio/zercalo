use super::animatable::Animatable;
use crate::scene::{Camera, HasBounding, HasCamera, HasMutCamera, HasMutScene, HasScene, Scene};
use glam::Vec3;

/// Allows update given value each frame by saved closure
pub struct Stepper<T> {
    pub value: T,
    pub stepper: Box<dyn FnMut(&mut T, u32)>,
}

impl<T> Stepper<T> {
    pub fn new<F>(value: T, stepper: F) -> Self
    where
        F: FnMut(&mut T, u32) + 'static,
    {
        Stepper {
            value,
            stepper: Box::new(stepper),
        }
    }
}
impl<T: HasCamera> HasCamera for Stepper<T> {
    fn get_camera(&self) -> &Camera {
        self.value.get_camera()
    }
}

impl<T: HasMutCamera> HasMutCamera for Stepper<T> {
    fn get_mut_camera(&mut self) -> &mut Camera {
        self.value.get_mut_camera()
    }
}

impl<T: HasBounding> HasBounding for Stepper<T> {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.value.get_bounding_volume()
    }
}

impl<T: HasScene> HasScene for Stepper<T> {
    fn get_scene(&self) -> &Scene {
        self.value.get_scene()
    }
}

impl<T: HasMutScene> HasMutScene for Stepper<T> {
    fn get_scene_mut(&mut self) -> &mut Scene {
        self.value.get_scene_mut()
    }
}

impl<T: Animatable> Animatable for Stepper<T> {
    fn animate(&mut self, frame: u32) {
        self.value.animate(frame);
        (self.stepper)(&mut self.value, frame);
    }
}
