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
fn test_setting_both_fullscreen_and_minimised_true_sets_both_to_false() {
    let state: WindowState = WindowState::new("Test".to_string(), true, true, true);

    assert!(!state.is_fullscreen());
    assert!(!state.is_minimised());
}