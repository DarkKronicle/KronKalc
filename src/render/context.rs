use glyphon::Resolution;
use wgpu::{Device, Queue};
use crate::render::FontData;

pub struct RenderContext {

    pub device: Device,
    pub queue: Queue,
    pub font_data: FontData,
    pub res: Resolution

}