use std::f64::consts::PI;

use software_render::{
    buffers::Buffer,
    game::{App, Game},
    math::vectors::Vector2,
    window_state::WindowState,
};
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
};

struct MyGame {
    scale: u32,
    events: Vec<WindowEvent>,
    should_close: bool,
    resize: Option<PhysicalSize<u32>>,

    framebuffer: Buffer<u32>,
}

impl MyGame {
    fn new() -> Self {
        Self {
            events: Vec::new(),
            should_close: false,
            resize: None,
            scale: 4,
            framebuffer: Buffer::new((300, 300).into(), 0xFFFFFFFF).unwrap(),
        }
    }

    fn resize(&mut self, state: &mut WindowState, size: PhysicalSize<u32>) {
        let size_vec = Vector2::new(
            (size.width / self.scale) as i32,
            (size.height / self.scale) as i32,
        );

        state.resize(size);
        self.framebuffer = Buffer::new(size_vec, 0xFFFFFFFFu32).unwrap();
    }

    fn render(&mut self) {
        self.framebuffer.clear(0xFFFFFFFFu32);

        let center = Vector2::new(
            self.framebuffer.width() as i32 / 2,
            self.framebuffer.height() as i32 / 2,
        );

        let side = 20;

        if center.x + side >= self.framebuffer.width() as i32
            || center.y + side >= self.framebuffer.height() as i32
        {
            return;
        }

        let rects = [Rect::new(
            (center.x - side, center.y - side).into(),
            (center.x + side, center.y + side).into(),
        )];

        let render_rects = rects.into_iter().filter_map(|r| r);

        for rect in render_rects {
            for pixel in rect {
                self.framebuffer.set_pixel(pixel, 0xFFFF00FF);
            }
        }

        let lines = [XLine::new(Vector2::new(0, 0), center)];
        let render_lines = lines.into_iter().filter_map(|l| l);

        for line in render_lines {
            for pixel in line {
                self.framebuffer.set_pixel(pixel, 0xFFFF00FF);
            }
        }
    }
}

impl Game for MyGame {
    fn new_event(&mut self, event: winit::event::WindowEvent) {
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

    fn update(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        state: &mut software_render::window_state::WindowState,
    ) {
        if self.should_close {
            event_loop.exit();
            return;
        }

        if let Some(size) = self.resize {
            self.resize(state, size);
        }

        self.render();
        state.draw(&self.framebuffer, self.scale);
    }
}

struct XLine {
    start: Vector2<i32>,
    end: Vector2<i32>,
    delta: Vector2<i32>,
    position: Vector2<i32>,
    increment: Vector2<i32>,
    module: i32,
}

impl XLine {
    fn new(start: Vector2<i32>, end: Vector2<i32>) -> Option<Self> {
        let (start, end) = if start.x < end.x {
            (start, end)
        } else {
            (end, start)
        };

        let delta = end - start;

        if delta.x < delta.y.abs() {
            return None;
        }

        Some(Self {
            start,
            end,
            delta,
            position: Vector2::new(0, 0),
            increment: Vector2::new(delta.x.signum(), delta.y.signum()),
            module: 0,
        })
    }
}

impl Iterator for XLine {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.x > self.delta.x {
            return None;
        }
        let output = self.position;

        self.position.x += self.increment.x;

        self.module += self.delta.y.abs();

        if self.module > self.delta.x.abs() {
            self.position.y += self.increment.y;
            self.module %= self.delta.x.abs();
        }

        Some(output)
    }
}

struct Rect {
    start: Vector2<i32>,
    end: Vector2<i32>,
    position: i32,
    position_end: i32,
    delta: Vector2<i32>,
}

impl Rect {
    fn new(start: Vector2<i32>, end: Vector2<i32>) -> Option<Self> {
        let delta = end - start;

        if delta.x < 1 || delta.y < 1 {
            return None;
        }

        Some(Self {
            start,
            end,
            position: 0,
            position_end: delta.x * delta.y,
            delta,
        })
    }
}

impl Iterator for Rect {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.position_end {
            return None;
        }

        let coords = Vector2::new(self.position % self.delta.y, self.position / self.delta.y);

        self.position += 1;

        Some(self.start + coords)
    }
}

fn main() {
    let my_game = MyGame::new();

    let mut app = App::new(Box::new(my_game));

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run_app(&mut app).unwrap();
}
