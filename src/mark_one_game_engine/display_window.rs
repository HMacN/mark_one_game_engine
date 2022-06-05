use pixels::{Pixels, SurfaceTexture};
use winit::{
    event_loop::EventLoop, window::WindowBuilder,
};
use winit::dpi::{LogicalSize, Size};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::window::{Fullscreen, Window};
use winit_input_helper::WinitInputHelper;

pub struct DisplayWindow
{

}

impl DisplayWindow
{
    pub fn new() -> Option<DisplayWindow>
    {
        return Option::Some(DisplayWindow {  })
    }

    pub fn display_window(&self)
    {
        let event_loop = EventLoop::new();
        let fullscreen = Option::from(Fullscreen::Borderless(None));

        let window_builder = WindowBuilder::new()
            .with_title("Hugh's Window")
            .with_fullscreen(fullscreen)
            ;

        let window: Window = window_builder.build(&event_loop).unwrap();

        let surface_texture = SurfaceTexture::new(window.inner_size().width, window.inner_size().height, &window);

        let mut input = WinitInputHelper::new();

        //let mut pixels = Pixels::new(window.inner_size().width, window.inner_size().height, surface_texture).unwrap();

        let event_handler = move |event: winit::event::Event<'_, ()>, c: &EventLoopWindowTarget<()>, control_flow: &mut winit::event_loop::ControlFlow|
        {
            // Instruct loop to begin again immediately, even if there are no new events.  This makes
            // the game continue to render to the screen.
            *control_flow = ControlFlow::Poll;

            if input.update(&event)
            {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Enter and exit fullscreen
                if input.key_pressed(VirtualKeyCode::F11)
                {
                    if window.fullscreen().is_some()
                    {
                        window.set_fullscreen(None)
                    }
                    else
                    {
                        window.set_fullscreen(Option::from(Fullscreen::Borderless(None)))
                    }

                    window.request_redraw();
                }

                // Resize the window
                if let Some(size) = input.window_resized() {
                    //pixels.resize_surface(size.width, size.height);
                }

                window.request_redraw();
            }
        };

        event_loop.run(event_handler);
    }
}