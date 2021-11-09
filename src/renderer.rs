use pollster::block_on;
use wgpu::{Device, Queue, Surface};
use winit::window::Window;

pub struct Renderer<'window> {
    _surface: Surface,
    _device: Device,
    _queue: Queue,
    phantom_window: std::marker::PhantomData<&'window Window>,
}

impl<'window> Renderer<'window> {
    pub fn new(window: &'window Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let _surface = unsafe { instance.create_surface(window) };

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&_surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (_device, _queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("gltut Device"),
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
            },
            None,
        ))
        .unwrap();

        let size = window.inner_size();
        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_preferred_format(&adapter).unwrap(),
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo,
            },
        );

        Renderer {
            _surface,
            _device,
            _queue,
            phantom_window: std::marker::PhantomData,
        }
    }
}
