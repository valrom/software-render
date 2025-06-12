use std::rc::Rc;

use winit::{dpi::PhysicalSize, window::Window};

use crate::{buffers::Buffer, math::vectors::Vector2};

pub struct WindowState {
    pub window: Rc<Window>,
    pub surface: softbuffer::Surface<Rc<Window>, Rc<Window>>,
    pub size: PhysicalSize<u32>,
}

impl WindowState {
    pub fn new(window: &Rc<Window>, size: PhysicalSize<u32>) -> Option<Self> {
        let context = softbuffer::Context::new(window.clone()).ok()?;
        let mut surface = softbuffer::Surface::new(&context, window.clone()).ok()?;

        surface
            .resize(size.width.try_into().ok()?, size.height.try_into().ok()?)
            .ok()?;

        Some(Self {
            window: window.clone(),
            surface,
            size,
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> Option<()> {
        self.size = size;
        self.surface
            .resize(size.width.try_into().ok()?, size.height.try_into().ok()?)
            .ok()
    }

    pub fn draw(&mut self, framebuffer: &Buffer<u32>, scale: u32) -> Option<()> {
        let mut buffer = self.surface.buffer_mut().ok()?;

        let min_height = std::cmp::min(framebuffer.height() * scale, self.size.height);
        let min_width = std::cmp::min(framebuffer.width() * scale, self.size.width);

        for y in 0..min_height {
            for x in 0..min_width {
                buffer[(self.size.width * y + x) as usize] =
                    framebuffer.get_pixel(Vector2::new(x as i32, y as i32) / scale as i32)
            }
        }

        buffer.present().ok()?;
        self.window.request_redraw();

        Some(())
    }
}
