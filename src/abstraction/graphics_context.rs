use winit::{event_loop::{EventLoop, self}, window::{Window, WindowBuilder}, dpi::{PhysicalSize, Size}};

pub struct GraphicsContext;

impl GraphicsContext
{
    pub fn init()
    {
        env_logger::init();
    }

    pub fn create_window<S : AsRef<str>>(title : S, width : u32, height : u32) -> (EventLoop<()>, Window)
    {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title(title.as_ref()).with_inner_size(Size::Physical(PhysicalSize::new(width, height))).build(&event_loop).expect("Window Creation Failed");

        (event_loop, window)
    }
} 