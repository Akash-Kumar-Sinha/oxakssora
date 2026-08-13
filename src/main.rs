mod renderer;
mod window;

fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    let mut app = window::App::default();

    event_loop.run_app(&mut app).unwrap();
}
