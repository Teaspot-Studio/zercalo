use crate::scene::Scene;

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
