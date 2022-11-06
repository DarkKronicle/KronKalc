use glyphon::{
    fontdue::{
        Font, FontSettings,
    },
    Color, HasColor, Resolution, TextAtlas, TextRenderer,
};
use wgpu::{CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, TextureViewDescriptor};
use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window},
};
use winit_input_helper::WinitInputHelper;
use crate::layout::context::RenderContext;
use crate::layout::renderable::Renderable;
use crate::line::line_handler::LineHandler;
use crate::screen::Screen;

pub const BACKGROUND_COLOR: wgpu::Color = wgpu::Color::BLACK;

#[derive(Clone, Copy)]
struct GlyphUserData;

impl HasColor for GlyphUserData {
    fn color(&self) -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

pub struct FontStorage {
    pub fonts: Vec<Font>
}

pub struct FontData {
    pub atlas: TextAtlas,
    pub renderer: TextRenderer,
    pub font_storage: FontStorage
}

pub async fn run(event_loop: EventLoop<()>, window: Window) {
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // Prepare swap chain
    let swapchain_format = surface.get_supported_formats(&adapter)[0];
    let size = window.inner_size();

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: surface.get_supported_alpha_modes(&adapter)[0],
    };

    surface.configure(&device, &config);

    let atlas = TextAtlas::new(&device, &queue, swapchain_format);
    let text_renderer = TextRenderer::new(&device, &queue);

    let font = include_bytes!("../assets/resources/fonts/JetBrainsMono-Regular.ttf") as &[u8];
    let font = Font::from_bytes(font, FontSettings::default()).unwrap();
    let fonts = vec![font];

    let font_data = FontData {
        atlas,
        renderer: text_renderer,
        font_storage: FontStorage { fonts }
    };

    let screen: Screen = Screen {
        line_handler: LineHandler::default()
    };

    let mut input = WinitInputHelper::new();

    let mut context = RenderContext {
        device,
        queue,
        font_data,
        res: Resolution {
            width: config.width,
            height: config.height,
        }
    };

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            return;
        }
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                config.width = size.width;
                config.height = size.height;
                surface.configure(&context.device, &config);
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {



                screen.render(&mut context);

                let frame = surface.get_current_texture().unwrap();
                let view = frame.texture.create_view(&TextureViewDescriptor::default());
                let mut encoder =
                    context.device.create_command_encoder(&CommandEncoderDescriptor { label: None });
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

                    context.font_data.renderer.render(&context.font_data.atlas, &mut pass).unwrap();
                }

                context.queue.submit(Some(encoder.finish()));
                frame.present();
            }
            _ => {}
        }
    });
}