use glyphon::{
    fontdue::{
        Font, FontSettings,
    },
    Resolution, TextAtlas, TextRenderer,
};
use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window},
};
use winit_input_helper::WinitInputHelper;
use crate::layout::lines::Lines;
use crate::layout::screen::Screen;
use crate::render::context::RenderContext;
use crate::render::{FontData, FontStorage};
use crate::render::renderer::Renderer;

pub const BACKGROUND_COLOR: wgpu::Color = wgpu::Color::BLACK;

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
        line_handler: Lines::default()
    };

    let mut input = WinitInputHelper::new();

    let context = RenderContext {
        device,
        queue,
        font_data,
        res: Resolution {
            width: config.width,
            height: config.height,
        }
    };

    let mut renderer = Renderer {
        context,
        screen,
        surface
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
                renderer.surface.configure(&renderer.context.device, &config);
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                renderer.render();
            }
            _ => {}
        }
    });
}