use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use glyphon::{Color, TextOverflow};
use crate::render::context::RenderContext;
use crate::render::Renderable;
use crate::text::literal_text::{Format, LiteralText};

pub mod literal_text;

pub struct Text {
    components: Vec<LiteralText>,
    x: f32,
    y: f32
}

impl Text {

    pub fn of(literal: LiteralText, x: f32, y: f32) -> Text {
        Text {
            components: vec![literal],
            x,
            y
        }
    }

    pub fn height(&self) -> f32 {
        let mut max: f32 = 0f32;
        for t in self.components.iter() {
            if t.format.size > max {
                max = t.format.size;
            }
        }
        return max;
    }

}

impl Default for Text {

    fn default() -> Self {
        Text {
            components: vec![LiteralText::of(String::from("Hello there!"), Color {r: 255, g: 255, b: 255, a: 255}, 30.)],
            x: 2f32,
            y: 2f32
        }
    }

}

impl LayoutAble for Text {

    fn get_layout(&self, context: &mut RenderContext, x: f32, y: f32) -> Layout<Format> {
        let mut layout: Layout<Format> = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x,
            y,
            ..LayoutSettings::default()
        });
        for t in self.components.iter() {
            layout.append(
                context.font_data.font_storage.fonts.as_slice(),
                &TextStyle::with_user_data(
                    &t.content,
                    t.format.size,
                    0,
                    t.format,
                ),
            );
        }
        return layout;
    }

}

impl Renderable for Text {
    fn render(&mut self, context: &mut RenderContext) {

        let mut layouts: Vec<(Layout<Format>, TextOverflow)> = Vec::<(Layout<Format>, TextOverflow)>::new();
        let mut y: f32 = self.y;
        for l in self.components.iter() {
            let layout = l.get_layout(context, self.x, y);
            y += l.format.size + 2.;
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


pub trait LayoutAble {

    fn get_layout(&self, context: &mut RenderContext, x: f32, y: f32) -> Layout<Format>;

}