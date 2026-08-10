use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize, Position, Size},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowAttributes,
};

#[derive(Default)]
struct App {
    window: Option<Arc<winit::window::Window>>,
}

impl ApplicationHandler for App {
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

        self.window = Some(window)
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut app = App::default();

    event_loop.run_app(&mut app).unwrap();
}
