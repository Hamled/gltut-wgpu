mod renderer;

use renderer::Renderer;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct App {
    #[allow(dead_code)]
    window: Window,
    pub renderer: Renderer,
}

impl App {
    fn new(window: Window) -> Self {
        let renderer = Renderer::new(&window);
        Self { window, renderer }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut app = App::new(window);

    event_loop.run(move |_event, _target, control_flow| {
        app.renderer.clear_immediate(wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        });
        *control_flow = ControlFlow::Wait;
    });
}
