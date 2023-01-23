
use crate::view::window_state::WindowState;
use crate::view::winit_adapter::WinItAdapter;

pub struct MarkOneWindow {
    window: WinItAdapter,
    state: WindowState,
}

impl MarkOneWindow {
    pub fn new() -> MarkOneWindow {
        return MarkOneWindow {
            window: WinItAdapter::new(),
            state: WindowState::default_new(),
        }
    }

    pub fn run_window(&mut self) {
        self.window.run_window(self.state.clone())
    }

    fn set_window_state(&mut self) {
        self.window.set_window_state(self.state.clone())
    }

    fn retrieve_window_state(&mut self) {
        self.state = self.window.get_window_state();
    }

    pub fn show_window(&mut self) {
        self.retrieve_window_state();
        self.state.show();
        self.set_window_state();
    }

    pub fn hide_window(&mut self) {
        self.retrieve_window_state();
        self.state.hide();
        self.set_window_state();
    }

    pub fn enter_fullscreen(&mut self) {
        self.retrieve_window_state();
        self.state.go_fullscreen();
        self.set_window_state();
    }

    pub fn exit_fullscreen(&mut self) {
        self.retrieve_window_state();
        self.state.exit_fullscreen();
        self.set_window_state();
    }

    pub fn minimise(&mut self) {
        self.retrieve_window_state();
        self.state.minimise();
        self.set_window_state();
    }

    pub fn restore_from_minimised(&mut self) {
        self.retrieve_window_state();
        self.state.restore();
        self.set_window_state();
    }

    pub fn set_window_title(&mut self, title: String) {
        self.retrieve_window_state();
        self.state.set_title(title);
        self.set_window_state();
    }
}