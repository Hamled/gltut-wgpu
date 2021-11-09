use pollster::block_on;
use std::rc::Rc;
use wgpu::{Color, Device, Queue, Surface};
use winit::window::Window;

pub struct Renderer {
    #[allow(dead_code)]
    window: Rc<Window>,
    surface: Surface,
    device: Device,
    queue: Queue,
}

impl Renderer {
    pub fn new(window: Rc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let surface = unsafe { instance.create_surface(&*window) };

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = block_on(adapter.request_device(
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
            window,
            surface,
            device,
            queue,
        }
    }

    // Clear the color buffer immediately
    pub fn clear_immediate(&mut self, color: Color) {
        let texture = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Timeout) => return,
            Err(e) => panic!("Got error from get_current_texture(): {}", e),
        };

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("clear_immediate"),
            });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear_immediate"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default()),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        self.queue.submit(std::iter::once(encoder.finish()));
        texture.present();
    }
}
