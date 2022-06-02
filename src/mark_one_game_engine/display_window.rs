use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize, event::VirtualKeyCode, event_loop::EventLoop, window::WindowBuilder,
};
use winit::dpi::{Position, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::monitor::VideoMode;
use winit::window::{Fullscreen, Window};

pub struct DisplayWindow
{
    file_name: String
}

impl DisplayWindow
{
    pub fn new(file_name: &str) -> Option<DisplayWindow>
    {
        return Option::Some(DisplayWindow { file_name: file_name.to_string() })
    }

    pub fn display_window(&self)
    {
        let event_loop = EventLoop::new();
        let fullscreen = Option::from(Fullscreen::Borderless(None));

        let mut window_builder = WindowBuilder::new()
            .with_title("Hugh's Window")
            .with_fullscreen(fullscreen)
            ;

        let window: Window = window_builder.build(&event_loop).unwrap();

        window.request_redraw();

        event_loop.run(move |event, _, mut control_flow|
            {
                *control_flow = ControlFlow::Poll;

                match event
                {
                    Event::WindowEvent
                    {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {*control_flow = ControlFlow::Exit },
                    _ => ()
                }
            })
    }

    // pub fn display(&self)
    // {
    //
    //     let WIDTH= 512;
    //     let HEIGHT = 256;
    //     let event_loop = EventLoop::new();
    //
    //     let window =
    //     {
    //         let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    //         let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
    //         WindowBuilder::new()
    //             .with_title("pixel invaders")
    //             .with_inner_size(scaled_size)
    //             .with_min_inner_size(size)
    //             .build(&event_loop)
    //             .unwrap()
    //     };
    //
    //     let pixels =
    //     {
    //         let window_size = window.inner_size();
    //         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    //         Pixels::new(WIDTH, HEIGHT, surface_texture)
    //     };
    // }
}