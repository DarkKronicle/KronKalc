use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use glyphon::{Color, HasColor};
use crate::layout::context::RenderContext;


pub trait LayoutAble {

    fn get_layout(&self, context: &mut RenderContext, x: f32, y: f32) -> Layout<Format>;

}

#[derive(Clone)]
pub struct Text {
    pub content: String,
    pub format: Format,
}

#[derive(Copy, Clone)]
pub struct Format {
    pub color: Color,
    pub size: f32
}

impl Text {
    pub fn of(content: String, color: Color, size: f32) -> Self {
        Text {
            content,
            format: Format {
                color,
                size
            }
        }
    }
}

impl HasColor for Format {
    fn color(&self) -> Color {
        return self.color;
    }
}

impl LayoutAble for Text {

    fn get_layout(&self, context: &mut RenderContext, x: f32, y: f32) -> Layout<Format> {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x,
            y,
            ..LayoutSettings::default()
        });

        layout.append(
            context.font_data.font_storage.fonts.as_slice(),
            &TextStyle::with_user_data(
                &self.content,
                self.format.size,
                0,
                self.format
            ),
        );

        return layout;
    }


}