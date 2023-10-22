use wgpu::{Backends, Surface, Device, Queue, SurfaceConfiguration};
use winit::{event_loop::EventLoop, window::Window};

use super::graphics_context::GraphicsContext;

pub struct HighLevelGraphicsContext;

impl HighLevelGraphicsContext
{
    pub async fn init_window<S : AsRef<str>>(title : S, width : u32, height : u32) -> (EventLoop<()>, Window, Surface, Device, Queue, SurfaceConfiguration)
    {
        let (event_loop, window) = GraphicsContext::create_window(title, width, height);
        
        let size = window.inner_size();
        let instance = GraphicsContext::create_instance(Backends::all(), Default::default());
        
        let surface = GraphicsContext::create_surface(&window, &instance).expect("Error when creating surface for ProspectWindow");

        let adapter = GraphicsContext::create_adapter(&instance, &surface).await.expect("Error when creating adapter for ProspectWindow");

        let (device, queue) = GraphicsContext::create_device(&adapter).await.expect("Error while creating device for ProspectWindow");

        let config = GraphicsContext::config_surface_easy(&surface, &adapter, &device, (size.width, size.height));

        (event_loop, window, surface, device, queue, config)
    }
}