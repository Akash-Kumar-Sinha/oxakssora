use crate::renderer::Renderer;

use log::info;
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize, Position, Size},
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::WindowAttributes,
};

#[derive(Default)]
pub struct App<'a> {
    window: Option<Arc<winit::window::Window>>,
    renderer: Option<Renderer<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_inner_size(Size::Physical(PhysicalSize::new(800, 600)))
            .with_title("oxakssora")
            .with_resizable(true)
            .with_position(Position::Physical(PhysicalPosition::new(0, 0)));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let renderer = Renderer::new(Arc::clone(&window));

        self.window = Some(window);
        self.renderer = Some(renderer);

        self.get_window().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed");
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(size.width, size.height);
                }

                self.get_window().request_redraw();
            }

            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.render();
                }

                self.get_window().request_redraw();
            }

            _ => {}
        }
    }
}

impl<'a> App<'a> {
    fn get_window(&self) -> &Arc<winit::window::Window> {
        self.window.as_ref().unwrap()
    }
}
