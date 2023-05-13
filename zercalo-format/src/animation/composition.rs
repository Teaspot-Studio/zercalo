use super::animatable::Animatable;
use glam::f32::Quat;
use glam::Vec3;

pub struct RelativePart<T> {
    pub value: T,
    pub position: Vec3,
    pub rotation: Quat,
}

impl<T: Animatable> Animatable for RelativePart<T> {
    fn animate(&mut self, frame: u32) {
        self.value.animate(frame)
    }
}

pub struct Composition<T> {
    pub parts: Vec<RelativePart<T>>,
    pub position: Vec3,
    pub rotation: Quat,
}

impl<T: Animatable> Animatable for Composition<T> {
    fn animate(&mut self, frame: u32) {
        for m in self.parts.iter_mut() {
            m.animate(frame);
        }
    }
}
