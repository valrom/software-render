mod math;
mod raster;
mod triangles;

use image::open;
use image::ImageBuffer;
use image::Rgb;
use math::matrices::Matrix4;
use math::vectors::Vector3;
use math::vectors::Vector4;
use raster::Triangle;
use std::ops::Add;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowAttributes;
use winit::window::{Window, WindowId};

use crate::math::vectors::Vector2;
use softbuffer::Buffer;

struct App {
    state: Option<State>,
    w: u32,
    h: u32,
    time: std::time::SystemTime,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

#[allow(dead_code)]
struct RenderContext {
    width: u32,
    height: u32,

    framebuffer: Vec<u32>,
    depthbuffer: Vec<u8>,
}

#[allow(dead_code)]
impl RenderContext {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            framebuffer: vec![0; (width * height) as usize],
            depthbuffer: vec![u8::max_value(); (width * height) as usize],
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.width = new_size.width;
        self.height = new_size.height;

        self.framebuffer = vec![0; (self.width * self.height) as usize];
        self.depthbuffer = vec![0; (self.width * self.height) as usize];
    }

    fn clean(&mut self, color: u32) {
        self.framebuffer.fill(color);
        self.depthbuffer.fill(u8::max_value());
    }

    fn draw(&mut self, x: u32, y: u32, z: u8, color: u32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let depth = self.depthbuffer.get_mut((y * self.width + x) as usize);
        let Some(depth) = depth else {
            return false;
        };

        if z > *depth {
            return false;
        }

        let write_color = self.framebuffer.get_mut((y * self.width + x) as usize);
        let Some(write_color) = write_color else {
            return false;
        };

        *depth = z;
        *write_color = color;

        true
    }

    fn framebuffer(&self) -> &[u32] {
        self.framebuffer.as_ref()
    }
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

    fn draw<F>(&mut self, draw: F)
    where
        F: Fn(&mut Drawer),
    {
        let buffer = self.surface.buffer_mut().unwrap();
        let mut drawer = Drawer {
            size: self.size,
            buffer,
            pixel_size: 2,
        };

        draw(&mut drawer);
        Self::update(&self.window, drawer.buffer)
    }

    fn update(window: &Rc<Window>, buffer: Buffer<Rc<Window>, Rc<Window>>) {
        buffer.present().unwrap();
        window.request_redraw();
    }
}

struct Drawer<'a> {
    buffer: Buffer<'a, Rc<Window>, Rc<Window>>,
    size: PhysicalSize<u32>,
    pixel_size: u32,
}

impl<'a> Drawer<'a> {
    fn draw_pixel(&mut self, x: u32, y: u32, color: u32) -> bool {
        let begin_x = x * self.pixel_size;
        let begin_y = y * self.pixel_size;

        if begin_x >= self.size.width - self.pixel_size
            || begin_y >= self.size.height - self.pixel_size
        {
            return false;
        }

        let range_x = begin_x..begin_x + self.pixel_size;
        let range_y = begin_y..begin_y + self.pixel_size;

        for x in range_x {
            for y in range_y.clone() {
                self.buffer[(y * self.size.width + x) as usize] = color;
            }
        }

        true
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

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, another: Self) -> Self {
        Self {
            r: (self.r + another.r) / 2.0,
            g: (self.g + another.g) / 2.0,
            b: (self.b + another.b) / 2.0,

            a: (self.a + another.a) / 2.0,
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

            WindowEvent::RedrawRequested => state.draw(|draw| {
                let w = self.w as i32 / draw.pixel_size as i32;
                let h = self.h as i32 / draw.pixel_size as i32;

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

                let aspect = if h != 0 && w != 0 {
                    w as f32 / h as f32
                } else {
                    1.0
                };

                let projection = Matrix4::projection(aspect, 3.14 / 2.0, 0.1, 100.0);

                let mut look = Matrix4::identity();
                look.z.w = -1.5;

                let viewport = Matrix4::viewport(Vector2::new(w, h));

                let matrix = viewport * projection * look * rotate;

                let triangle_iter = mesh
                    .into_iter()
                    .map(|indecies| {
                        (
                            vertices[indecies.0],
                            vertices[indecies.1],
                            vertices[indecies.2],
                        )
                    })
                    .filter_map(|triangle| {
                        let ndc = [
                            matrix * triangle.0.position,
                            matrix * triangle.1.position,
                            matrix * triangle.2.position,
                        ];

                        let iter = Triangle::new(ndc)?.into_iter();
                        Some((iter, triangle))
                    });

                for (iter, triangle) in triangle_iter {
                    for i in iter {
                        if i.position.x > 0.0
                            && i.position.x < w as f32
                            && i.position.y > 0.0
                            && i.position.y < h as f32
                        {
                            let _color = triangle.0.color * i.coefs.x
                                + triangle.1.color * i.coefs.y
                                + triangle.2.color * i.coefs.z;

                            let uvs = triangle.0.uv * i.coefs.x
                                + triangle.1.uv * i.coefs.y
                                + triangle.2.uv * i.coefs.z;

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

                            draw.draw_pixel(i.position.x as u32, i.position.y as u32, final_color);
                        }
                    }
                }
            }),

            WindowEvent::Resized(size) => {
                state.resize(size);
                self.w = size.width;
                self.h = size.height;
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
        w: 0,
        h: 0,
        time: std::time::SystemTime::now(),
        image,
    };
    event_loop.run_app(&mut app).unwrap();
}

fn make_window(elwt: &ActiveEventLoop) -> Rc<Window> {
    Rc::new(elwt.create_window(WindowAttributes::default()).unwrap())
}
