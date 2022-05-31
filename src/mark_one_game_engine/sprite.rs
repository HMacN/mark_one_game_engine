use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize, event::VirtualKeyCode, event_loop::EventLoop, window::WindowBuilder,
};

pub struct Sprite
{
    file_name: String
}

impl Sprite
{
    pub fn new(file_name: &str) -> Option<Sprite>
    {
        return Option::Some(Sprite{ file_name: file_name.to_string() })
    }

    pub fn display(&self)
    {

        let WIDTH= 512;
        let HEIGHT = 256;
        let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
            WindowBuilder::new()
                .with_title("pixel invaders")
                .with_inner_size(scaled_size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let pixels =
            {
                let window_size = window.inner_size();
                let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
                Pixels::new(WIDTH, HEIGHT, surface_texture);
            };

        pixels.
    }
}