use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

mod renderer;
mod layout;
mod line;
mod text;
mod screen;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("KronKalc").build(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Temporarily avoid srgb formats for the swapchain on the web
        pollster::block_on(renderer::run(event_loop, window));
    }

}