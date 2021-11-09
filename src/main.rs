use std::rc::Rc;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod renderer;

fn main() {
    let event_loop = EventLoop::new();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());

    let mut renderer = renderer::Renderer::new(window.clone());

    event_loop.run(move |_event, _target, control_flow| {
        renderer.clear_immediate(wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        });
        *control_flow = ControlFlow::Wait;
    });
}
