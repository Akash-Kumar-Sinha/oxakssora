use std::sync::Arc;

use log::info;

use wgpu::InstanceDescriptor;
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize, Position, Size},
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::WindowAttributes,
};

#[derive(Default)]
pub(crate) struct App<'a> {
    window: Option<Arc<winit::window::Window>>,
    surface: Option<Arc<wgpu::Surface<'a>>>,
    adapter: Option<Arc<wgpu::Adapter>>,
    instance: Option<Arc<wgpu::Instance>>,
    device: Option<Arc<wgpu::Device>>,
    queue: Option<Arc<wgpu::Queue>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes: WindowAttributes = WindowAttributes::default()
            .with_inner_size(Size::Physical(PhysicalSize::new(800, 600)))
            .with_title("oxakssora".to_string())
            .with_resizable(true)
            .with_position(Position::Physical(PhysicalPosition::new(0, 0)));

        let window = {
            let window = ActiveEventLoop::create_window(event_loop, window_attributes).unwrap();
            Arc::new(window)
        };

        let display = Arc::clone(&window);

        let instance = Arc::new(wgpu::Instance::new(
            InstanceDescriptor::new_with_display_handle(Box::new(display)),
        ));

        let adapter = Arc::new(
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                .unwrap(),
        );

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        self.instance = Some(instance);
        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);

        self.window = Some(window);

        self.surface_config();

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            
            WindowEvent::RedrawRequested => {
                let surface = self.get_surface();
                let device = self.get_device();
                let queue = self.get_queue();

                match surface.get_current_texture() {
                    wgpu::CurrentSurfaceTexture::Success(output) => {
                        let view = output
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: Some("Render Encoder"),
                            });

                        Self::render_screen_background(
                            &mut encoder,
                            &view,
                            wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.0,
                            },
                        );

                        queue.submit(Some(encoder.finish()));

                        queue.present(output);
                    }

                    wgpu::CurrentSurfaceTexture::Occluded
                    | wgpu::CurrentSurfaceTexture::Timeout => (),

                    wgpu::CurrentSurfaceTexture::Suboptimal(output) => {
                        let view = output
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: Some("Render Encoder"),
                            });

                        Self::render_screen_background(
                            &mut encoder,
                            &view,
                            wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 1.0,
                                a: 1.0,
                            },
                        );

                        queue.submit(Some(encoder.finish()));

                        queue.present(output);
                    }

                    wgpu::CurrentSurfaceTexture::Outdated => {
                        self.surface_config();
                    }

                    wgpu::CurrentSurfaceTexture::Validation => {
                        unreachable!("No error scope registered, so validation errors will panic")
                    }

                    wgpu::CurrentSurfaceTexture::Lost => {
                        self.surface_config();
                    }
                }
            }

            _ => {}
        }
    }
}

impl<'a> App<'a> {
    pub fn get_window(&self) -> &Arc<winit::window::Window> {
        self.window.as_ref().unwrap()
    }

    pub fn get_surface(&self) -> &Arc<wgpu::Surface<'a>> {
        self.surface.as_ref().unwrap()
    }

    pub fn get_adapter(&self) -> &Arc<wgpu::Adapter> {
        self.adapter.as_ref().unwrap()
    }

    pub fn get_device(&self) -> &Arc<wgpu::Device> {
        self.device.as_ref().unwrap()
    }

    pub fn get_queue(&self) -> &Arc<wgpu::Queue> {
        self.queue.as_ref().unwrap()
    }

    pub fn get_instance(&self) -> &Arc<wgpu::Instance> {
        self.instance.as_ref().unwrap()
    }

    pub fn surface_config(&mut self) {
        let instance = self.get_instance();
        let adapter = self.get_adapter();
        let device = self.get_device();
        let window = Arc::clone(self.get_window());

        let surface = instance.create_surface(window).unwrap();
        let surface = Arc::new(surface);

        let capabilities = surface.get_capabilities(adapter);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width: 800,
            height: 600,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Srgb,
        };

        surface.configure(device, &surface_config);

        self.surface = Some(surface);
    }

    pub fn render_screen_background(
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        color: wgpu::Color,
    ) {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Background Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }
}
