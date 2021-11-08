use wgpu::{Instance, Surface};
use winit::window::Window;

pub struct Renderer<'window> {
    _surface: Surface,
    phantom_window: std::marker::PhantomData<&'window Window>,
}

impl<'window> Renderer<'window> {
    pub fn new(window: &'window Window) -> Self {
        let instance = Instance::new(wgpu::Backends::VULKAN);
        let _surface = unsafe { instance.create_surface(window) };

        Renderer {
            _surface,
            phantom_window: std::marker::PhantomData,
        }
    }
}
