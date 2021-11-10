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
        app.renderer.ch1_draw();

        *control_flow = ControlFlow::Wait;
    });
}
