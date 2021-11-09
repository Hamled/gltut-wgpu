use std::rc::Rc;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod renderer;

fn main() {
    let event_loop = EventLoop::new();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());

    let _renderer = renderer::Renderer::new(window.clone());

    event_loop.run(move |_event, _target, control_flow| {
        *control_flow = ControlFlow::Wait;
    });
}
