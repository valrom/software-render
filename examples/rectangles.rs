use std::f64::consts::PI;

use software_render::{
    buffers::Buffer,
    drawing::line::Line,
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

    mouse_position: Vector2<i32>,
}

impl MyGame {
    fn new() -> Self {
        Self {
            events: Vec::new(),
            should_close: false,
            resize: None,
            scale: 4,
            framebuffer: Buffer::new((300, 300).into(), 0xFFFFFFFF).unwrap(),
            mouse_position: Vector2::new(0, 0),
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

        let side = 1;

        // if center.x + side >= self.framebuffer.width() as i32
        //     || center.y + side >= self.framebuffer.height() as i32
        // {
        //     return;
        // }

        // let rects = [Rect::new(
        //     (center.x - side, center.y - side).into(),
        //     (center.x + side, center.y + side).into(),
        // )];

        // let render_rects = rects.into_iter().filter_map(|r| r);

        // for rect in render_rects {
        //     for pixel in rect {
        //         self.framebuffer.set_pixel(pixel, 0xFFFF00FF);
        //     }
        // }

        let lines = [Line::new(self.mouse_position, center)];
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

        for event in self.events.iter() {
            if let WindowEvent::CursorMoved { position, .. } = event {
                self.mouse_position = Vector2::new(
                    (position.x / self.scale as f64) as i32,
                    (position.y / self.scale as f64) as i32,
                );
            };
        }

        self.mouse_position.x = self
            .mouse_position
            .x
            .clamp(0, self.framebuffer.width() as i32 - 1);
        self.mouse_position.y = self
            .mouse_position
            .y
            .clamp(0, self.framebuffer.height() as i32 - 1);

        self.render();
        state.draw(&self.framebuffer, self.scale);
    }
}

fn main() {
    let my_game = MyGame::new();

    let mut app = App::new(Box::new(my_game));

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run_app(&mut app).unwrap();
}
