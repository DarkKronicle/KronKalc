use crate::layout::context::RenderContext;

pub trait Renderable {
    fn render(&self, context: &mut RenderContext);
}