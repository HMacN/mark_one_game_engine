use crate::view::mark_one_window::MarkOneWindow;

pub struct MarkOneGameCore {
    window: MarkOneWindow,
}

impl MarkOneGameCore {
    pub fn display_window(&mut self) {
        self.window.run("Materiel");
    }
}

impl MarkOneGameCore {

    pub fn new() -> MarkOneGameCore {
        return MarkOneGameCore { window: MarkOneWindow::new() };
    }
}