use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use glyphon::Color;
use crate::layout::context::RenderContext;
use crate::text::{Format, Text};
use crate::text::LayoutAble;

pub struct LineContent {
    text: Vec<Text>
}

impl LineContent {

    pub fn height(&self) -> f32 {
        let mut max: f32 = 0f32;
        for t in self.text.iter() {
            if t.format.size > max {
                max = t.format.size;
            }
        }
        return max;
    }

    pub fn of(content: String, color: Color, size: f32) -> Self {
        LineContent {
            text: vec![Text::of(content, color, size)]
        }
    }

}

impl LayoutAble for LineContent {

    fn get_layout(&self, context: &mut RenderContext, x: f32, y: f32) -> Layout<Format> {
        let mut layout: Layout<Format> = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x,
            y,
            ..LayoutSettings::default()
        });
        for t in self.text.iter() {
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