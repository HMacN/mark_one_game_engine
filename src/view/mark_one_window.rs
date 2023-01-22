use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Fullscreen, Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub struct MarkOneWindow {
    input: WinitInputHelper,
    window: Option<Window>,
    state: WindowState,
}

impl MarkOneWindow {
    pub fn new() -> MarkOneWindow {
        return MarkOneWindow {
            input: WinitInputHelper::new(),
            window: None,
            state: WindowState::new(),
        }
    }

    pub fn run_window(&mut self) {
        let event_loop: EventLoop<()> = EventLoop::new();
        self.window = Option::from(WindowBuilder::new().build(&event_loop).unwrap());

        self.update_window_state();

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

    fn update_window_state(&self) {
        if self.window.is_none() {
            return;
        } else {
            let window: &Window = self.window.as_ref().unwrap();
            self.pass_window_state_to_window(window)
        }
    }

    fn pass_window_state_to_window(&self, window: &Window) {
        window.set_title(&*self.state.title.clone());
        window.set_visible(self.state.visible);
        window.set_fullscreen(self.state.full_screen.clone());
        window.set_minimized(self.state.minimised)
    }

    pub fn show_window(&mut self) {
        self.state.visible = true;
        self.update_window_state();
    }

    pub fn hide_window(&mut self) {
        self.state.visible = false;
        self.update_window_state();
    }

    pub fn enter_fullscreen(&mut self) {
        self.state.full_screen = Option::from(Fullscreen::Borderless(None));
        self.update_window_state();
    }

    pub fn exit_fullscreen(&mut self) {
        self.state.full_screen = None;
        self.update_window_state();
    }

    pub fn enter_minimised(&mut self) {
        self.state.minimised = true;
        self.update_window_state();
    }

    pub fn exit_minimised(&mut self) {
        self.state.minimised = false;
        self.update_window_state();
    }

    pub fn set_window_title(&mut self, title: String) {
        self.state.title = title;
        self.update_window_state();
    }

    pub fn close(&mut self) {
        self.window = None;
        self.state = WindowState::new();
    }
}


struct WindowState {
    title: String,
    visible: bool,
    full_screen: Option<Fullscreen>,
    minimised: bool,
}

impl WindowState {
    fn new() -> WindowState {
        return WindowState {
            title: "".to_string(),
            visible: false,
            full_screen: None,
            minimised: false,
        }
    }
}