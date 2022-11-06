use crate::layout::lines::Lines;
use crate::render::context::RenderContext;
use crate::render::Renderable;

pub struct Screen {

    pub line_handler: Lines

}

impl Renderable for Screen {

    fn render(&mut self, context: &mut RenderContext) {
        self.line_handler.render(context);
    }

}