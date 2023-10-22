use wgpu::{Backends, Surface, Device, Queue, SurfaceConfiguration, SurfaceTexture, Texture, TextureView, CommandEncoder};
use winit::{event_loop::EventLoop, window::Window};

use super::{graphics_context::GraphicsContext, prospect_window::ProspectWindow};

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

    pub fn start_render(window : &ProspectWindow, clear : (f64, f64, f64)) -> (SurfaceTexture, TextureView, CommandEncoder)
    {
        let (output, view) = GraphicsContext::create_view(window.get_surface());
        let mut command_encoder = GraphicsContext::create_command_encoder(window.get_device(), "Draw Loop Commands");
        let render_pass = GraphicsContext::begin_render_pass_barebones((clear.0, clear.1, clear.2, 1.0), "Render Pass", &view, &mut command_encoder);
        drop(render_pass);

        (output, view, command_encoder)
    }

    pub fn finish_render(window : &ProspectWindow, command_encoder : CommandEncoder, output : SurfaceTexture)
    {
        window.get_queue().submit(std::iter::once(command_encoder.finish()));
        output.present()
    }
}