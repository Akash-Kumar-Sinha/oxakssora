mod pipeline;
mod render_item;
mod scene;

use std::sync::Arc;

use render_item::RenderItem;
use scene::Scene;

pub struct Renderer<'a> {
    surface: Arc<wgpu::Surface<'a>>,
    adapter: Arc<wgpu::Adapter>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,

    pipeline: wgpu::RenderPipeline,

    scene: Scene,

    width: u32,
    height: u32,
}

impl<'a> Renderer<'a> {
    pub fn new(window: Arc<winit::window::Window>) -> Self {
        let display = Arc::clone(&window);

        let instance = Arc::new(wgpu::Instance::new(
            wgpu::InstanceDescriptor::new_with_display_handle(Box::new(display)),
        ));

        let adapter = Arc::new(
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                .unwrap(),
        );

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let surface = Arc::new(instance.create_surface(window).unwrap());

        let capabilities = surface.get_capabilities(&adapter);

        let format = capabilities.formats[0];

        let width = 800;
        let height = 600;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Srgb,
        };

        surface.configure(&device, &config);

        let pipeline = pipeline::create_pipeline(&device, format);

        let mut scene = Scene::new();

        // Add Components to the scene
        scene.add(RenderItem::Rectangle);

        Self {
            surface,
            adapter,
            device,
            queue,
            pipeline,
            scene,
            width,
            height,
        }
    }

    pub fn render(&mut self) {
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => output,

            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                self.reconfigure();
                return;
            }

            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Suboptimal(output) => output,

            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("Validation error");
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = components::bg_cycle::bg_cycle(&mut encoder, &view);

            render_pass.set_pipeline(&self.pipeline);

            self.scene.render(&self.device, &mut render_pass);
        }

        self.queue.submit(Some(encoder.finish()));

        self.queue.present(output);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.width = width;
        self.height = height;

        self.reconfigure();
    }

    fn reconfigure(&self) {
        let capabilities = self.surface.get_capabilities(&self.adapter);

        let format = capabilities.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,

            format,

            width: self.width,
            height: self.height,

            present_mode: wgpu::PresentMode::Fifo,

            alpha_mode: capabilities.alpha_modes[0],

            view_formats: vec![],

            desired_maximum_frame_latency: 2,

            color_space: wgpu::SurfaceColorSpace::Srgb,
        };

        self.surface.configure(&self.device, &config);
    }
}
