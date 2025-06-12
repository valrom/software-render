use std::rc::Rc;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

use crate::window_state::WindowState;

pub trait Game {
    fn new_event(&mut self, event: WindowEvent);

    fn clear_events(&mut self);

    fn update(&mut self, event_loop: &ActiveEventLoop, state: &mut WindowState);
}

pub struct App {
    state: Option<WindowState>,
    game: Box<dyn Game>,
}

impl App {
    pub fn new(game: Box<dyn Game>) -> Self {
        Self { state: None, game }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = make_window(event_loop);
        self.state = WindowState::new(&window, PhysicalSize::new(300, 300));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(mut state) = self.state.take() else {
            return;
        };

        if id != state.window.id() {
            self.state.replace(state);
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                self.game.update(event_loop, &mut state);
                self.game.clear_events();
            }

            _ => {
                self.game.new_event(event);
            }
        }

        self.state.replace(state);
    }
}

pub fn make_window(elwt: &ActiveEventLoop) -> Rc<Window> {
    Rc::new(elwt.create_window(WindowAttributes::default()).unwrap())
}
