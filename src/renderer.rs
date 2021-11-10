use pollster::block_on;
use safe_transmute::{transmute_to_bytes, TriviallyTransmutable};
use wgpu::{util::DeviceExt, Color, Device, Queue, Surface, SurfaceConfiguration};
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
    #[allow(dead_code)]
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

    // Chapter 1 triangle example
    pub fn ch1_draw(&mut self) {
        let shader = {
            let desc = wgpu::ShaderModuleDescriptor {
                label: Some("Ch1 Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/ch1.wgsl").into()),
            };

            self.device.create_shader_module(&desc)
        };

        let pipeline = {
            let layout = {
                let desc = wgpu::PipelineLayoutDescriptor {
                    label: Some("Ch1 Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                };

                self.device.create_pipeline_layout(&desc)
            };

            let vert_buffer_layout = wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vector4>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x4],
            };

            let targets = [wgpu::ColorTargetState {
                format: self.config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }];

            let desc = wgpu::RenderPipelineDescriptor {
                label: Some("Ch1 Pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[vert_buffer_layout],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &targets,
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false,
                },
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                depth_stencil: None,
            };

            self.device.create_render_pipeline(&desc)
        };

        let verts = {
            let verts = vec![
                vec4(0.75, 0.75, 0.0, 1.0),
                vec4(0.75, -0.75, 0.0, 1.0),
                vec4(-0.75, -0.75, 0.0, 1.0),
            ];

            let desc = wgpu::util::BufferInitDescriptor {
                label: Some("Ch1 Vertex Buffer"),
                usage: wgpu::BufferUsages::VERTEX,
                contents: transmute_to_bytes(verts.as_slice()),
            };

            self.device.create_buffer_init(&desc)
        };

        let texture = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Timeout) => return,
            Err(e) => panic!("Got error from get_current_texture(): {}", e),
        };

        let output = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Ch1 Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Ch1 Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(Color::BLACK),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        // Draw the vertices with simple indices, 1 instance
        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, verts.slice(..));
        render_pass.draw(0..3, 0..1);

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        texture.present();
    }
}
