pub struct WindowState {
    title: String,
    visible: bool,
    full_screen: bool,
    minimised: bool,
    maximised: bool,
}

impl WindowState {
    pub fn default_new() -> WindowState {
        return WindowState {
            title: "".to_string(),
            visible: false,
            full_screen: true,
            minimised: false,
            maximised: false
        }
    }

    pub fn new(title: String, visible: bool, full_screen: bool, minimised: bool, maximised: bool) -> WindowState {
        return WindowState {
            title,
            visible,
            full_screen,
            minimised,
            maximised,
        }
    }

    pub fn get_title(&self) -> String {
        return self.title.clone();
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn is_visible(&self) -> bool {
        return self.visible;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn is_fullscreen(&self) -> bool {
        return self.full_screen;
    }

    pub fn go_fullscreen(&mut self) {
        self.full_screen = true;
    }

    pub fn exit_fullscreen(&mut self) {
        self.full_screen = false;
    }

    pub fn is_minimised(&self) -> bool {
        return self.minimised;
    }

    pub fn is_maximised(&self) -> bool {
        return self.maximised
    }

    pub fn minimise(&mut self) {
        self.minimised = true;
    }

    pub fn maximise(&mut self) {
        self.maximised = true;
    }

    pub fn restore(&mut self) {
        self.minimised = false;
    }

    pub fn clone(&self) -> WindowState {
        return WindowState {
            title: self.title.clone(),
            visible: self.visible,
            full_screen: self.full_screen,
            minimised: self.minimised,
            maximised: self.maximised
        }
    }
}
