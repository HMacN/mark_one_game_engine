use std::cmp::min;
use mark_one_game_engine::view::window_state;
use mark_one_game_engine::view::window_state::WindowState;

#[test]
fn test_initialises_with_blank_title_string() {
    let state: WindowState = WindowState::default_new();
    let title: String = state.get_title();

    assert_eq!("", title);
}

#[test]
fn test_initialise_with_fullscreen_set_to_true() {
    let state: WindowState = WindowState::default_new();
    let full_screen: bool = state.is_fullscreen();

    assert!(full_screen);
}

#[test]
fn test_initialise_with_visible_set_to_false() {
    let state: WindowState = WindowState::default_new();
    let visible: bool = state.is_visible();

    assert!(!visible);
}

#[test]
fn test_initialise_with_minimised_set_to_false() {
    let state: WindowState = WindowState::default_new();
    let minimised: bool = state.is_minimised();

    assert!(!minimised);
}

#[test]
fn test_initialise_with_variables() {
    let state: WindowState = WindowState::new("Test".to_string(), true, false, true);

    let title: String = state.get_title();
    let visible: bool = state.is_visible();
    let full_screen: bool = state.is_fullscreen();
    let minimised: bool = state.is_minimised();

    assert_eq!("Test".to_string(), title);
    assert!(visible);
    assert!(!full_screen);
    assert!(minimised);
}

#[test]
fn test_clone_function() {
    let state_1: WindowState = WindowState::new("Test".to_string(), true, false, true);
    let state_2: WindowState = state_1.clone();

    assert_eq!(state_1.get_title(), state_2.get_title());
    assert_eq!(state_1.is_visible(), state_2.is_visible());
    assert_eq!(state_1.is_fullscreen(), state_2.is_fullscreen());
    assert_eq!(state_1.is_minimised(), state_2.is_minimised());
}

#[test]
fn test_set_title() {
    let mut state: WindowState = WindowState::default_new();
    let new_title: String = "New Title".to_string();

    state.set_title(new_title.clone());

    assert_eq!(new_title, state.get_title());
}

#[test]
fn test_show() {
    let mut state: WindowState = WindowState::new("Test".to_string(), false, false, true);

    state.show();

    assert!(state.is_visible());
}

#[test]
fn test_hide() {
    let mut state: WindowState = WindowState::new("Test".to_string(), false, false, true);

    state.hide();

    assert!(!state.is_visible());
}

#[test]
fn test_minimise() {
    let mut state: WindowState = WindowState::new("Test".to_string(), false, false, false);

    state.minimise();

    assert!(state.is_minimised());
}

#[test]
fn test_restore_from_minimised() {
    let mut state: WindowState = WindowState::new("Test".to_string(), false, false, true);

    state.restore();

    assert!(!state.is_minimised());
}

#[test]
fn test_go_fullscreen() {
    let mut state: WindowState = WindowState::new("Test".to_string(), false, false, false);

    state.go_fullscreen();

    assert!(state.is_fullscreen());
}

#[test]
fn test_exit_fullscreen() {
    let mut state: WindowState = WindowState::new("Test".to_string(), false, true, true);

    state.exit_fullscreen();

    assert!(!state.is_fullscreen());
}