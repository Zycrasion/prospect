use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}, event, dpi::{Size, LogicalSize, PhysicalSize}};

use super::graphics_context::GraphicsContext;

pub struct ProspectWindow
{
    event_loop : EventLoop<()>,
    window : Window
}

impl ProspectWindow
{
    pub fn new<S : AsRef<str>>(title : S, width : u32, height : u32) -> Self
    {
        let (event_loop, window) = GraphicsContext::create_window(title, width, height);
        Self
        {
            event_loop,
            window
        }
    }
}