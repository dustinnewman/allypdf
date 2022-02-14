use crate::operators::engine::GraphicsState;

pub trait Render {
    fn stroke(state: GraphicsState);
    fn fill(state: GraphicsState);
}
