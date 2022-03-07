use crate::scene::{Model, Scene};

/// Trait that allows to wrap scene into more specific type and add animation state.
/// Renderer need to know only final scene state and how to step the scene between frames.
pub trait Animatable {
    /// Step scene for the next frame
    fn animate(&mut self, frame: u32);
}

/// We always can render static scene
impl Animatable for Scene {
    fn animate(&mut self, _frame: u32) {}
}

/// We always can render static model
impl Animatable for Model {
    fn animate(&mut self, _frame: u32) {}
}
