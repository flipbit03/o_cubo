use std::error::Error;

use app::DesktopRenderer;
use winit::event_loop::EventLoop;

mod app;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();

    let mut app = DesktopRenderer::new();

    event_loop.run_app(&mut app).unwrap();

    Ok(())
}
