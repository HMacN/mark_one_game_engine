use std::cmp::min;
use winit::window::{Fullscreen, Window, WindowBuilder};
use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent};
use crate::view::window_state::WindowState;

pub(crate) struct WinItAdapter {
    window: Option<Window>,
    title: String,
    minimised: bool,
}

impl WinItAdapter {
    pub fn new() -> WinItAdapter {
        return WinItAdapter {
            window: None,
            title: "".to_string(),
            minimised: false,
        }
    }

    pub fn run_window(&mut self, state: WindowState) {
        let event_loop: EventLoop<()> = EventLoop::new();
        self.window = Option::from(WindowBuilder::new().build(&event_loop).unwrap());

        self.set_window_state(state);

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();    // Sets the loop to continuously update, even if there are no events from the OS.

            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    control_flow.set_exit();
                },
                _ => {}
            }
        });
    }

    pub fn set_window_state(&mut self, state: WindowState) {
        if self.window.is_some() {
            let window: &Window = self.window.as_ref().unwrap();
            window.set_title(&*state.get_title());
            window.set_visible(state.is_visible());
            window.set_minimized(state.is_minimised());

            if state.is_fullscreen() {
                window.set_fullscreen(Option::from(Fullscreen::Borderless(None)))
            } else {
                window.set_fullscreen(None)
            }
        }

        self.title = state.get_title();
        self.minimised = state.is_minimised();
    }

    fn update_title(&mut self, window: &Window, title: String) {
        self.title = title.clone();
        window.set_title(&*title.clone());
    }

    fn update_minimised(&mut self, window: &Window, minimised: bool) {
        self.minimised = minimised;
        window.set_minimized(minimised);
    }

    pub fn get_window_state(&self) -> WindowState {

        let window: &Window = self.window.as_ref().unwrap();

        return WindowState::new(
            self.title.clone(),
            WinItAdapter::get_visible_from_window(window),
            WinItAdapter::get_fullscreen_from_window(window),
            self.minimised,
        )
    }

    fn get_visible_from_window(window: &Window) -> bool {
        return if window.is_visible().is_none() {
            false
        } else {
            window.is_visible().unwrap()
        }
    }

    fn get_fullscreen_from_window(window: &Window) -> bool {
        return if window.fullscreen().is_none() {
            false
        } else {
            true
        }
    }
}
