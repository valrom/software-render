use image::open;
use image::ImageBuffer;
use image::Rgb;
use software_render::buffers::Buffer;
use software_render::color::Color;
use software_render::game::App;
use software_render::game::Game;
use software_render::math::matrices::Matrix4;
use software_render::math::vectors::Vector3;
use software_render::math::vectors::Vector4;
use software_render::raster::Triangle;
use software_render::window_state::WindowState;
use std::collections::HashSet;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowAttributes;
use winit::window::{Window, WindowId};

use software_render::math::vectors::Vector2;

struct MyGame {
    state: Option<WindowState>,

    framebuffer: Buffer<u32>,
    depth: Buffer<f32>,

    time: std::time::SystemTime,
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    scale: u32,

    events: Vec<WindowEvent>,

    should_close: bool,
    resize: Option<PhysicalSize<u32>>,
}

impl Game for MyGame {
    fn new_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.should_close = true;
            }

            WindowEvent::Resized(size) => {
                if size.width / self.scale == 0 || size.height / self.scale == 0 {
                    return;
                }

                self.resize = Some(size);
            }

            _ => self.events.push(event),
        }
    }

    fn clear_events(&mut self) {
        self.events.clear();
    }

    fn update(&mut self, event_loop: &ActiveEventLoop, state: &mut WindowState) {
        if self.should_close {
            event_loop.exit();
        }

        if let Some(size) = self.resize {
            self.resize(state, size);
        }

        self.draw();
        state.draw(&self.framebuffer, self.scale);
    }
}

impl MyGame {
    fn resize(&mut self, state: &mut WindowState, size: PhysicalSize<u32>) {
        let size_vec = Vector2::new(
            (size.width / self.scale) as i32,
            (size.height / self.scale) as i32,
        );

        state.resize(size);

        self.framebuffer = Buffer::new(size_vec, 0xFFFFFFFFu32).unwrap();
        self.depth = Buffer::new(size_vec, -1.0).unwrap();
    }
}

impl MyGame {
    fn draw(&mut self) {
        self.framebuffer.clear(0xFFFFFFFF);
        self.depth.clear(-1.0);

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

        let cube = [
            Vector3::new(-1.0, 1.0, -1.0),
            Vector3::new(1.0, 1.0, -1.0),
            Vector3::new(-1.0, 1.0, 1.0),
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(1.0, -1.0, -1.0),
            Vector3::new(-1.0, -1.0, 1.0),
            Vector3::new(1.0, -1.0, 1.0),
        ];

        let uvs = [
            Vector2::new(0.0, 0.0),
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(1.0, 1.0),
        ];

        pub struct Index {
            pub position: usize,
            pub uv: usize,
            pub color: Color,
        }

        impl Index {
            fn new(position: usize, uv: usize, color: Color) -> Self {
                Self {
                    position,
                    uv,
                    color,
                }
            }
        }

        fn quad(indexs: [usize; 4], color: Color) -> [(Index, Index, Index); 2] {
            [
                (
                    Index::new(indexs[0], 0, color),
                    Index::new(indexs[1], 1, color),
                    Index::new(indexs[2], 2, color),
                ),
                (
                    Index::new(indexs[2], 2, color),
                    Index::new(indexs[1], 1, color),
                    Index::new(indexs[3], 3, color),
                ),
            ]
        }

        let quads = [
            quad([0, 2, 7, 5], Color::from_rgb(1.0, 0.0, 0.0)),
            quad([2, 0, 5, 7], Color::from_rgb(1.0, 0.0, 0.0)),
            quad([0, 1, 6, 7], Color::from_rgb(0.0, 1.0, 0.0)),
            quad([1, 0, 7, 6], Color::from_rgb(0.0, 1.0, 0.0)),
        ];

        let mesh = quads.as_flattened();

        let time = self.time.elapsed().unwrap().as_secs_f32();
        let rotate = Matrix4::<f32>::rotation_x(-time);

        let aspect = if self.framebuffer.width() != 0 && self.framebuffer.height() != 0 {
            self.framebuffer.width() as f32 / self.framebuffer.height() as f32
        } else {
            1.0
        };

        let projection = Matrix4::projection(aspect, 3.14 / 2.0, 0.01, 0.2);

        let mut look = Matrix4::identity();
        look.z.w = -3.0;

        let viewport = Matrix4::viewport(Vector2::new(
            self.framebuffer.width() as i32,
            self.framebuffer.height() as i32,
        ));

        let matrix = viewport * projection * look * rotate;

        let triangle_iter = mesh.into_iter().filter_map(|indices| {
            let triangle = (
                Vertex {
                    position: cube[indices.0.position].into(),
                    color: indices.0.color.into(),
                    uv: uvs[indices.0.uv],
                },
                Vertex {
                    position: cube[indices.1.position].into(),
                    color: indices.0.color.into(),
                    uv: uvs[indices.1.uv],
                },
                Vertex {
                    position: cube[indices.2.position].into(),
                    color: indices.0.color.into(),
                    uv: uvs[indices.2.uv],
                },
            );

            let ndc = [
                matrix * triangle.0.position,
                matrix * triangle.1.position,
                matrix * triangle.2.position,
            ];

            let iter = Triangle::new(ndc)?.into_iter();
            Some((iter, triangle))
        });

        let mut values = HashSet::new();

        for (iter, triangle) in triangle_iter {
            for frag in iter {
                if frag.position.x > 0.0
                    && frag.position.x < self.framebuffer.width() as f32
                    && frag.position.y > 0.0
                    && frag.position.y < self.framebuffer.height() as f32
                    && frag.position.z < 0.0
                    && frag.position.z > -1.0
                {
                    let pixel_pos = Vector2::new(frag.position.x as i32, frag.position.y as i32);
                    let depth = frag.position.z;

                    let get_depth = self.depth.get_pixel(pixel_pos);

                    if get_depth > depth {
                        continue;
                    }

                    values.insert((depth * 10000.0) as i32);

                    self.depth.set_pixel(pixel_pos, depth);

                    let color = frag.coefs.interpolate((
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

                    let Some(texture) = self
                        .image
                        .get_pixel_checked(texture.x as u32, texture.y as u32)
                    else {
                        continue;
                    };

                    self.framebuffer.set_pixel(
                        pixel_pos,
                        Color::from_rgb(color.x, color.y, color.z).to_u32(),
                    );
                    // self.framebuffer.set_pixel(pixel_pos, Color::from(*texture).to_u32());
                }
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let image = open("./textures/brick.png").unwrap().into_rgb8();

    let mut game = Box::new(MyGame {
        state: None,
        scale: 2,
        framebuffer: Buffer::new(Vector2::new(100, 100), 0xFFFFFFu32).unwrap(),
        depth: Buffer::new(Vector2::new(100, 100), 0.0).unwrap(),
        time: std::time::SystemTime::now(),
        image,
        events: Vec::new(),
        resize: None,
        should_close: false,
    });

    let mut app = App::new(game);

    event_loop.run_app(&mut app).unwrap();
}
