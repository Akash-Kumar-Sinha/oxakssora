use winit::event_loop::EventLoop;

use crate::window::App;

mod window;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut app = App::default();

    event_loop.run_app(&mut app).unwrap();
}
