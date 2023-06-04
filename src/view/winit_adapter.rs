use std::cmp::max;
use winit::window::{Fullscreen, Window, WindowBuilder};
use winit::event_loop::EventLoop;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit_input_helper::WinitInputHelper;
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
        let mut input_helper: WinitInputHelper = WinitInputHelper::new();
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

            if input_helper.update(&event) {
                // generate_input_notifications(input_helper)   // TODO: Generate non-WinIt events for the rest of the program from detected WinIt input events.
            };
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

    fn update_maximised(&mut self, window: &Window, maximised: bool) {
        window.set_maximized(maximised);
    }

    pub fn get_window_state(&self) -> Option<WindowState> {
        return if self.window.is_none() {
            None
        } else {
            let window: &Window = self.window.as_ref().unwrap();

            Option::from(WindowState::new(
                self.title.clone(),
                WinItAdapter::get_visible_from_window(window),
                WinItAdapter::get_fullscreen_from_window(window),
                self.minimised,
                WinItAdapter::get_maximised_from_window(window)))
        }
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

    fn get_maximised_from_window(window: &Window) -> bool {
        return window.is_maximized();
    }

    fn generate_input_notifications(&self, input_helper: WinitInputHelper) {
        // if input_helper.key_pressed(VirtualKeyCode::) {  // TODO: Convert input_helper input types into non-WinIt types.
        //
        // }
    }
}

struct Listener {

}

impl Listener {
    fn notify(&self) {

    }
}

enum InputEvent {   // todo add all keys and input events to this.
    PressedA,
    PressedB,
    PressedC,
    PressedD,
    PressedE,
    PressedF,
    PressedG,
    PressedH,
    PressedI,
    PressedJ,
    PressedK,
    PressedL,
    PressedM,
    PressedN,
    PressedO,
    PressedP,
    PressedQ,
    PressedR,
    PressedS,
    PressedT,
    PressedU,
    PressedV,
    PressedW,
    PressedX,
    PressedY,
    PressedZ,

}


