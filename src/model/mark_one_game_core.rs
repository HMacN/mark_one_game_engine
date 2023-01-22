use crate::view::mark_one_window::MarkOneWindow;

pub struct MarkOneGameCore {
    window: MarkOneWindow,
}

impl MarkOneGameCore {
    pub fn new() -> MarkOneGameCore {
        return MarkOneGameCore {
            window: MarkOneWindow::new()
        }
    }

    pub fn display_window(&mut self) {
        self.window.enter_fullscreen();
        self.window.exit_fullscreen();
        self.window.enter_minimised();
        self.window.exit_minimised();
        self.window.set_window_title(String::from("Test"));
        self.window.hide_window();
        self.window.show_window();

        self.window.run_window();   // Blocks for the lifetime of the window.
    }
}