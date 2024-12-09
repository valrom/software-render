mod buffers;
mod math;
mod raster;
mod triangles;

use buffers::Buffer;
use image::open;
use image::ImageBuffer;
use image::Rgb;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowAttributes;
use winit::window::{Window, WindowId};

use crate::math::vectors::Vector2;

struct App {
    state: Option<State>,
    buffer: Buffer<u32>,

    time: std::time::SystemTime,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    scale: u32,
}

struct State {
    window: Rc<Window>,
    surface: softbuffer::Surface<Rc<Window>, Rc<Window>>,
    size: PhysicalSize<u32>,
}

impl State {
    fn new(window: &Rc<Window>) -> Option<Self> {
        let context = softbuffer::Context::new(window.clone()).ok()?;
        let mut surface = softbuffer::Surface::new(&context, window.clone()).ok()?;
        let size = PhysicalSize::<u32>::new(300, 300);

        surface
            .resize(
                size.width.try_into().unwrap(),
                size.height.try_into().unwrap(),
            )
            .ok()?;
        Some(Self {
            window: window.clone(),
            surface,
            size,
        })
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.surface
            .resize(
                size.width.try_into().unwrap(),
                size.height.try_into().unwrap(),
            )
            .unwrap();
    }

    pub fn draw(&mut self, framebuffer: &Buffer<u32>, scale: u32) {
        let mut buffer = self.surface.buffer_mut().unwrap();

        for y in 0..std::cmp::min(framebuffer.height() * scale, self.size.height) {
            for x in 0..std::cmp::min(framebuffer.width() * scale, self.size.width) {
                buffer[(self.size.width * y + x) as usize] =
                    framebuffer.get_pixel(Vector2::new(x as i32, y as i32) / scale as i32)
            }
        }

        buffer.present().unwrap();
        self.window.request_redraw();
    }
}

#[derive(Copy, Clone)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[allow(dead_code)]
impl Color {
    fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    fn to_u32(self) -> u32 {
        0x01000000 * (self.a * 255.0).trunc() as u32
            + 0x00010000 * (self.r * 255.0).trunc() as u32
            + 0x00000100 * (self.g * 255.0).trunc() as u32
            + 0x00000001 * (self.b * 255.0).trunc() as u32
    }

    fn scale(self, factor: f32) -> Self {
        Self {
            a: self.a * factor,
            b: self.b * factor,
            r: self.r * factor,
            g: self.g * factor,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = make_window(event_loop);
        self.state = State::new(&window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(ref mut state) = self.state else {
            return;
        };

        if id != state.window.id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                state.draw(&self.buffer, self.scale);
            }

            WindowEvent::Resized(size) => {
                if size.width / self.scale == 0 || size.height / self.scale == 0 {
                    return;
                }

                state.resize(size);
                self.buffer = Buffer::new(
                    Vector2::new(
                        (size.width / self.scale) as i32,
                        (size.height / self.scale) as i32,
                    ),
                    0xFFFFFFFFu32,
                )
                .unwrap();
            }

            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let image = open("./textures/brick.jpg").unwrap().into_rgb8();

    let mut app = App {
        state: None,
        scale: 4,
        buffer: Buffer::new(Vector2::new(100, 100), 0xFFFFFFu32).unwrap(),
        time: std::time::SystemTime::now(),
        image,
    };
    event_loop.run_app(&mut app).unwrap();
}

fn make_window(elwt: &ActiveEventLoop) -> Rc<Window> {
    Rc::new(elwt.create_window(WindowAttributes::default()).unwrap())
}
