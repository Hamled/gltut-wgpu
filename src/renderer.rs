use pollster::block_on;
use safe_transmute::{transmute_to_bytes, TriviallyTransmutable};
use wgpu::{Color, Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

#[derive(Clone, Copy)]
struct Vector4(cgmath::Vector4<f32>);
unsafe impl TriviallyTransmutable for Vector4 {}

fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
    Vector4(cgmath::vec4(x, y, z, w))
}

pub struct Renderer {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let surface = unsafe { instance.create_surface(window) };

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
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Renderer {
            surface,
            device,
            queue,
            config,
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
