use crate::operators::engine::GraphicsState;

pub trait Output {
    fn stroke(&mut self, state: &GraphicsState);
    fn fill(&mut self, state: &GraphicsState);
}
