use fontdue::layout::Layout;
use glyphon::{Color, TextOverflow};
use crate::layout::context::RenderContext;
use crate::line::content::LineContent;
use crate::layout::renderable::Renderable;
use crate::text::{Format, LayoutAble};

pub struct LineHandler {
    lines: Vec<LineContent>,
}

impl Default for LineHandler {

    fn default() -> Self {
        LineHandler {
            lines: vec![LineContent::of(String::from("Hello there!"), Color {r: 255, g: 255, b: 255, a: 255}, 30.)]
        }
    }

}

impl Renderable for LineHandler {
    fn render(&self, context: &mut RenderContext) {

        let mut layouts: Vec<(Layout<Format>, TextOverflow)> = Vec::<(Layout<Format>, TextOverflow)>::new();
        let mut y: f32 = 0.;
        for l in self.lines.iter() {
            let layout = l.get_layout(context, 2f32, y);
            y += l.height() + 2.;
            let tup = (layout, TextOverflow::Hide);
            layouts.push(tup);
        }

        context.font_data.renderer
            .prepare(
                &context.device,
                &context.queue,
                &mut context.font_data.atlas,
                context.res,
                &context.font_data.font_storage.fonts,
                layouts.as_slice()
            )
            .unwrap();
    }
}
