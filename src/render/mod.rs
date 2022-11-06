use fontdue::Font;
use glyphon::{TextAtlas, TextRenderer};
use crate::render::context::RenderContext;

pub mod handler;
pub mod renderer;
pub mod context;

pub struct FontStorage {
    pub fonts: Vec<Font>
}

pub struct FontData {
    pub atlas: TextAtlas,
    pub renderer: TextRenderer,
    pub font_storage: FontStorage
}

pub trait Renderable {
    fn render(&mut self, context: &mut RenderContext);
}
