mod math;
mod triangles;

use image::open;
use image::ImageBuffer;
use image::Rgb;
use math::matrices::Matrix4;
use math::matrices::Vector3;
use math::matrices::Vector4;
use math::VecF2;
use math::VecI2;
use std::ops::Add;
use std::rc::Rc;
use triangles::RasterTriangle;
use triangles::Rasterizer;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowAttributes;
use winit::window::{Window, WindowId};

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
            pixel_size: 4,
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
                let w = self.w as i32 / 4;
                let h = self.h as i32 / 4;

                let texture_cube = (
                    (Vector3::new(-1.0, 1.0, 0.0), VecF2::new(0.0, 0.0)),
                    (Vector3::new(1.0, 1.0, 0.0), VecF2::new(1.0, 0.0)),
                    (Vector3::new(-1.0, -1.0, 0.0), VecF2::new(0.0, 1.0)),
                    (Vector3::new(1.0, -1.0, 0.0), VecF2::new(1.0, 1.0)),
                );

                let cube = (
                    (
                        Vector3::new(-1.0, 1.0, -1.0),
                        Color::from_rgb(1.0, 0.0, 0.0),
                    ),
                    (Vector3::new(1.0, 1.0, -1.0), Color::from_rgb(0.0, 1.0, 0.0)),
                    (
                        Vector3::new(-1.0, -1.0, -1.0),
                        Color::from_rgb(0.0, 0.0, 1.0),
                    ),
                    (
                        Vector3::new(1.0, -1.0, -1.0),
                        Color::from_rgb(0.4, 0.4, 0.4),
                    ),
                    (Vector3::new(-1.0, 1.0, 1.0), Color::from_rgb(0.5, 0.5, 0.5)),
                    (Vector3::new(1.0, 1.0, 1.0), Color::from_rgb(0.6, 0.6, 0.6)),
                    (
                        Vector3::new(-1.0, -1.0, 1.0),
                        Color::from_rgb(0.7, 0.7, 0.7),
                    ),
                    (Vector3::new(1.0, -1.0, 1.0), Color::from_rgb(0.8, 0.8, 0.8)),
                );

                let time = self.time.elapsed().unwrap().as_secs_f32();

                let rotation = Matrix4::rotation_x(time);

                let mut look = Matrix4::identity();
                look.w.z = 2.0;

                let aspect = if h != 0 && w != 0 {
                    w as f32 / h as f32
                } else {
                    1.0
                };

                let projection = Matrix4::projection(aspect, 3.14 / 2.0, 0.1, 100.0);
                let matrix = projection * look * rotation;

                let _triangles = vec![
                    (cube.0, cube.1, cube.2),
                    (cube.2, cube.1, cube.3),
                    (cube.1, cube.5, cube.3),
                    (cube.5, cube.7, cube.3),
                    (cube.0, cube.1, cube.4),
                    (cube.1, cube.5, cube.4),
                ];

                let textures = vec![
                    (texture_cube.0, texture_cube.1, texture_cube.2),
                    (texture_cube.2, texture_cube.1, texture_cube.3),
                ];

                let triangles = textures
                    .into_iter()
                    .filter_map(|tri| {
                        let ndc = (matrix * tri.0 .0, matrix * tri.1 .0, matrix * tri.2 .0);

                        let ws = (ndc.0.w, ndc.1.w, ndc.2.w);

                        let ndc = (ndc.0 / ws.0, ndc.1 / ws.1, ndc.2 / ws.2);

                        let ndc_to_pixel_space = |vertex: Vector4| {
                            Vector3::new(
                                (vertex.x + 1.0) * w as f32 / 2.0,
                                (1.0 - vertex.y) * h as f32 / 2.0,
                                vertex.z,
                            )
                        };

                        let screen_space = (
                            ndc_to_pixel_space(ndc.0),
                            ndc_to_pixel_space(ndc.1),
                            ndc_to_pixel_space(ndc.2),
                        );

                        let pixel_space = (
                            VecI2::new(screen_space.0.x as i32, screen_space.0.y as i32),
                            VecI2::new(screen_space.1.x as i32, screen_space.1.y as i32),
                            VecI2::new(screen_space.2.x as i32, screen_space.2.y as i32),
                        );

                        let raster_triangle: RasterTriangle = pixel_space.try_into().ok()?;
                        let rasterizer: Rasterizer =
                            Rasterizer::new_with_cropping(raster_triangle, (w, h).into())?;

                        Some(((tri.0 .1, tri.1 .1, tri.2 .1), rasterizer))
                    })
                    .collect::<Vec<((VecF2, VecF2, VecF2), Rasterizer)>>();

                for triangle in triangles {
                    for pixel in triangle.1 {
                        let coords = triangle.0 .0.scale(pixel.attributes.0)
                            + triangle.0 .1.scale(pixel.attributes.1)
                            + triangle.0 .2.scale(pixel.attributes.2);

                        let coords = (
                            (coords.x * self.image.width() as f32) as u32,
                            (coords.y * self.image.height() as f32) as u32,
                        );

                        let Some(color) = self.image.get_pixel_checked(coords.0, coords.1) else {
                            continue;
                        };

                        let test = color.0;

                        let final_color = 0xFF000000u32
                            + test[2] as u32
                            + ((test[1] as u32) << 8)
                            + ((test[0] as u32) << 16);

                        draw.draw_pixel(
                            pixel.position.x as u32,
                            pixel.position.y as u32,
                            final_color,
                        );
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

    let image = open("./textures/brick.png").unwrap().into_rgb8();

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
