use wgpu::{CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, Surface, TextureViewDescriptor};
use crate::layout::screen::Screen;
use crate::render::context::RenderContext;
use crate::render::handler::BACKGROUND_COLOR;
use crate::render::Renderable;

pub struct Renderer {
    pub context: RenderContext,
    pub screen: Screen,
    pub surface: Surface
}

impl Renderer {

    pub(crate) fn render(&mut self) {
        self.screen.render(&mut self.context);

        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder =
            self.context.device.create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(BACKGROUND_COLOR),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.context.font_data.renderer.render(&self.context.font_data.atlas, &mut pass).unwrap();
        }

        self.context.queue.submit(Some(encoder.finish()));
        frame.present();
    }

}