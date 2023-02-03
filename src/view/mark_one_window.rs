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
        let window_state: Option<WindowState> = self.window.get_window_state();

        if window_state.is_none() {
            return;
        } else {
            self.state = window_state.unwrap();
        }
    }

    pub fn show_window(&mut self) {
        fn show(state: &mut WindowState) {
            state.show()
        }

        self.alter_window_state(show)
    }

    pub fn hide_window(&mut self) {
        fn hide(state: &mut WindowState) {
            state.hide()
        }

        self.alter_window_state(hide)
    }

    pub fn enter_fullscreen(&mut self) {
        fn fullscreen(state: &mut WindowState) {
            state.go_fullscreen()
        }

        self.alter_window_state(fullscreen)
    }

    pub fn exit_fullscreen(&mut self) {
        fn leave_fullscreen(state: &mut WindowState) {
            state.exit_fullscreen()
        }

        self.alter_window_state(leave_fullscreen)
    }

    pub fn minimise(&mut self) {
        fn minimise(state: &mut WindowState) {
            state.minimise()
        }

        self.alter_window_state(minimise)

    }

    pub fn restore_from_minimised(&mut self) {
        fn restore(state: &mut WindowState) {
            state.restore()
        }

        self.alter_window_state(restore)
    }

    pub fn set_window_title(&mut self, title: String) {
        self.retrieve_window_state();
        self.state.set_title(title);
        self.set_window_state();
    }

    fn alter_window_state(&mut self, f: fn(&mut WindowState)){
        self.retrieve_window_state();
        let s: &mut WindowState = &mut self.state;
        f(s);
        self.set_window_state();
    }
}