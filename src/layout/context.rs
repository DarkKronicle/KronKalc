use glyphon::Resolution;
use wgpu::{Device, Queue};
use crate::renderer::FontData;

pub struct RenderContext {

    pub device: Device,
    pub queue: Queue,
    pub font_data: FontData,
    pub res: Resolution

}