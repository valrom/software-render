mod buffers;
mod math;
mod raster;
mod triangles;

use buffers::Buffer;
use image::open;
use image::ImageBuffer;
use image::Rgb;
use math::matrices::Matrix4;
use math::vectors::Vector3;
use math::vectors::Vector4;
use raster::Triangle;
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
        let taken_state = self.state.take();

        let Some(mut state) = taken_state else {
            return;
        };

        if id != state.window.id() {
            self.state.replace(state);
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                self.draw();
                state.draw(&self.buffer, self.scale);
            }

            WindowEvent::Resized(size) => {
                if size.width / self.scale == 0 || size.height / self.scale == 0 {
                    return;
                }

                let size_vec = Vector2::new(
                    (size.width / self.scale) as i32,
                    (size.height / self.scale) as i32,
                );
                state.resize(size);
                self.buffer = Buffer::new(size_vec, 0xFFFFFFFFu32).unwrap();
            }

            _ => (),
        }

        self.state.replace(state);
    }
}

impl App {
    fn draw(&mut self) {
        #[derive(Copy, Clone)]
        struct Vertex {
            pub position: Vector4<f32>,
            pub color: Vector3<f32>,
            pub uv: Vector2<f32>,
        }

        impl Vertex {
            fn new(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, u: f32, v: f32) -> Self {
                Self {
                    position: Vector4::new(x, y, z, 1.0),
                    color: Vector3::new(r, g, b),
                    uv: Vector2::new(u, v),
                }
            }
        }

        let vertices = [
            Vertex::new(-1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0),
            Vertex::new(1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            Vertex::new(-1.0, -1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0),
        ];

        let mesh = [(0, 1, 2), (2, 3, 0), (2, 1, 0), (0, 3, 2)];

        let time = self.time.elapsed().unwrap().as_secs_f32();
        let rotate = Matrix4::<f32>::rotation_x(-time);

        let aspect = if self.buffer.width() != 0 && self.buffer.height() != 0 {
            self.buffer.width() as f32 / self.buffer.height() as f32
        } else {
            1.0
        };

        let projection = Matrix4::projection(aspect, 3.14 / 2.0, 0.1, 100.0);

        let mut look = Matrix4::identity();
        look.z.w = -1.5;

        let viewport = Matrix4::viewport(Vector2::new(
            self.buffer.width() as i32,
            self.buffer.height() as i32,
        ));

        let matrix = viewport * projection * look * rotate;

        let triangle_iter = mesh.into_iter().filter_map(|indices| {
            let triangle = (
                vertices[indices.0],
                vertices[indices.1],
                vertices[indices.2],
            );

            let ndc = [
                matrix * triangle.0.position,
                matrix * triangle.1.position,
                matrix * triangle.2.position,
            ];

            let iter = Triangle::new(ndc)?.into_iter();
            Some((iter, triangle))
        });

        for (iter, triangle) in triangle_iter {
            for frag in iter {
                if frag.position.x > 0.0
                    && frag.position.x < self.buffer.width() as f32
                    && frag.position.y > 0.0
                    && frag.position.y < self.buffer.height() as f32
                {
                    let _color = frag.coefs.interpolate((
                        triangle.0.color,
                        triangle.1.color,
                        triangle.2.color,
                    ));

                    let uvs = frag
                        .coefs
                        .interpolate((triangle.0.uv, triangle.1.uv, triangle.2.uv));

                    let texture = Vector2::new(
                        uvs.x * self.image.width() as f32,
                        uvs.y * self.image.height() as f32,
                    );

                    let Some(color) = self
                        .image
                        .get_pixel_checked(texture.x as u32, texture.y as u32)
                    else {
                        continue;
                    };

                    let final_color = 0xFF000000u32
                        + color.0[2] as u32
                        + ((color.0[1] as u32) << 8)
                        + ((color.0[0] as u32) << 16);

                    self.buffer.set_pixel(
                        Vector2::new(frag.position.x as i32, frag.position.y as i32),
                        final_color,
                    );
                }
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let image = open("./textures/brick.jpg").unwrap().into_rgb8();

    let mut app = App {
        state: None,
        scale: 2,
        buffer: Buffer::new(Vector2::new(100, 100), 0xFFFFFFu32).unwrap(),
        time: std::time::SystemTime::now(),
        image,
    };
    event_loop.run_app(&mut app).unwrap();
}

fn make_window(elwt: &ActiveEventLoop) -> Rc<Window> {
    Rc::new(elwt.create_window(WindowAttributes::default()).unwrap())
}
