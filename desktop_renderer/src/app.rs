use std::{
    num::NonZeroU32,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use softbuffer::Surface;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

type SurfaceFromWinitWindow = Surface<Rc<Window>, Rc<Window>>;

pub struct DesktopRenderer {
    width: u32,
    height: u32,
    window: Option<Rc<winit::window::Window>>,
    surface: Option<SurfaceFromWinitWindow>,
}

impl DesktopRenderer {
    /// Create a new DesktopRenderer with default width and height.
    pub fn new() -> Self {
        DesktopRenderer {
            width: 640,
            height: 480,
            window: None,
            surface: None,
        }
    }
}

trait ConvertColorToU32 {
    fn to_u32_argb(&self) -> u32;
}

impl ConvertColorToU32 for (u8, u8, u8) {
    #[inline]
    /// Convert an (R, G, B) color triplet into ARGB format as u32
    fn to_u32_argb(&self) -> u32 {
        (self.0 as u32) << 16 | (self.1 as u32) << 8 | self.2 as u32
    }
}

#[inline]
fn put_pixel(buffer: &mut [u32], width: u32, x: usize, y: usize, color: u32) {
    buffer[x + width as usize * y] = color
}

impl ApplicationHandler for DesktopRenderer {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let size = LogicalSize::new(self.width, self.height);
        let window = Rc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Desktop Renderer")
                        //.with_min_inner_size(size)
                        //.with_max_inner_size(size)
                        .with_inner_size(size), //.with_resizable(false),
                )
                .unwrap(),
        );

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let (Some(width), Some(height)) =
                    (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                {
                    // Resize surface
                    self.surface
                        .as_mut()
                        .unwrap()
                        .resize(width, height)
                        .unwrap();

                    self.width = size.width;
                    self.height = size.height;
                }
            }
            WindowEvent::RedrawRequested => {
                let mut buffer = self.surface.as_mut().unwrap().buffer_mut().unwrap();
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                let d = now.as_millis() / 32 % 256;
                println!("D: {:?}", &d);
                for x in 0..self.width as usize {
                    for y in 0..self.height as usize {
                        let color = (198, 0, 148).to_u32_argb();
                        put_pixel(&mut buffer, self.width, x, y, color);
                    }
                }
                buffer.present().unwrap();
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}
