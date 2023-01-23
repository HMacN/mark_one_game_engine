use std::cmp::min;

pub struct WindowState {
    title: String,
    visible: bool,
    full_screen: bool,
    minimised: bool,
}

impl WindowState {
    pub fn default_new() -> WindowState {
        return WindowState {
            title: "".to_string(),
            visible: false,
            full_screen: true,
            minimised: false,
        }
    }

    pub fn new(title: String, visible: bool, mut full_screen: bool, mut minimised: bool) -> WindowState {
        return if full_screen && minimised {
            WindowState {
                title,
                visible,
                full_screen: false,
                minimised: false,
            }
        } else {
            WindowState {
                title,
                visible,
                full_screen,
                minimised,
            }
        }
    }

    pub fn get_title(&self) -> String {
        return self.title.clone();
    }

    pub fn is_visible(&self) -> bool {
        return self.visible;
    }

    pub fn is_fullscreen(&self) -> bool {
        return self.full_screen;
    }

    pub fn is_minimised(&self) -> bool {
        return self.minimised;
    }
}
