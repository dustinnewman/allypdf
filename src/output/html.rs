use super::output::Output;
use crate::operators::engine::GraphicsState;

pub struct HtmlOutput {}

impl Output for HtmlOutput {
    fn stroke(&mut self, state: &GraphicsState) {
        let path = &state.path;
    }

    fn fill(&mut self, state: &GraphicsState) {
        todo!()
    }
}
