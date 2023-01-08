use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::{Fullscreen, Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use crate::model::mark_one_game_core::MarkOneGameCore;

pub struct MarkOneWindow {
    input: WinitInputHelper,
}

impl MarkOneWindow {

    pub fn new() -> MarkOneWindow {
        return MarkOneWindow { input: WinitInputHelper::new() };
    }

    pub fn run(&mut self, title: &str) {
        let event_loop: EventLoop<()> = EventLoop::new();
        let fullscreen = Option::from(Fullscreen::Borderless(None));
        let window_builder: WindowBuilder = WindowBuilder::new()
            .with_title(title)
            .with_fullscreen(fullscreen)
            ;

        let window: Window = window_builder.build(&event_loop).expect("Could not create a WinIt Window!");

        window.set_visible(true);

        // let surface_texture = SurfaceTexture::new(window.inner_size().width, window.inner_size().height, &window);

        //let mut pixels = Pixels::new(window.inner_size().width, window.inner_size().height, surface_texture).unwrap();

        let event_handler = move |event: Event<'_, ()>, c: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow| {
                // Instruct loop to begin again immediately, even if there are no new events.  This makes
                // the game continue to render to the screen.
                *control_flow = ControlFlow::Poll;

                if self.input.update(&event) {
                    // Close events
                    if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.quit() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    // Enter and exit fullscreen
                    if self.input.key_pressed(VirtualKeyCode::F11) {
                        if window.fullscreen().is_some() {
                            window.set_fullscreen(None)
                        } else {
                            window.set_fullscreen(Option::from(Fullscreen::Borderless(None)))
                        }

                        window.request_redraw();
                    }

                    // Resize the window
                    if let Some(_size) = self.input.window_resized() {
                        //pixels.resize_surface(size.width, size.height);
                    }

                    window.request_redraw();
                }
            };

        //event_loop.run(event_handler);    //todo fix this
    }
}