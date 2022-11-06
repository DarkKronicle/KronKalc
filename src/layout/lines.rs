use fontdue::layout::Layout;
use glyphon::{Color, TextOverflow};
use crate::render::context::RenderContext;
use crate::render::Renderable;
use crate::text::{LayoutAble, Text};
use crate::text::literal_text::{Format, LiteralText};

pub struct Lines {
    lines: Vec<Text>,
}

impl Default for Lines {

    fn default() -> Self {
        Lines {
            lines: vec![Text::of(
                LiteralText::of(String::from("Hello there!"), Color {r: 255, g: 255, b: 255, a: 255}, 30.),
                2f32,
                2f32
            )]
        }
    }

}

impl Renderable for Lines {
    fn render(&mut self, context: &mut RenderContext) {

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