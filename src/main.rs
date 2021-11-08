use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |_event, _target, control_flow| {
        *control_flow = ControlFlow::Wait;
    });
}
