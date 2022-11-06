use crate::layout::context::RenderContext;
use crate::layout::renderable::Renderable;
use crate::line::line_handler::LineHandler;

pub struct Screen {

    pub(crate) line_handler: LineHandler

}

impl Renderable for Screen {

    fn render(&self, context: &mut RenderContext) {
        self.line_handler.render(context);
    }

}