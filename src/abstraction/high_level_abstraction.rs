use wgpu::{
    Backends, CommandEncoder, Device, Queue, RenderPass, Surface, SurfaceConfiguration,
    SurfaceTexture, TextureView, RenderPipeline, BindGroupLayout,
};
use winit::{event_loop::EventLoop, window::Window};

use super::{graphics_context::GraphicsContext, prospect_window::ProspectWindow, shader::ProspectShader};

pub struct HighLevelGraphicsContext;

impl HighLevelGraphicsContext {
    pub async fn init_window<S: AsRef<str>>(
        title: S,
        width: u32,
        height: u32,
    ) -> (
        EventLoop<()>,
        Window,
        Surface,
        Device,
        Queue,
        SurfaceConfiguration,
    ) {
        let (event_loop, window) = GraphicsContext::create_window(title, width, height);

        let size = window.inner_size();
        let instance = GraphicsContext::create_instance(Backends::all(), Default::default());

        let surface = GraphicsContext::create_surface(&window, &instance)
            .expect("Error when creating surface for ProspectWindow");

        let adapter = GraphicsContext::create_adapter(&instance, &surface)
            .await
            .expect("Error when creating adapter for ProspectWindow");

        let (device, queue) = GraphicsContext::create_device(&adapter)
            .await
            .expect("Error while creating device for ProspectWindow");

        let config = GraphicsContext::config_surface_easy(
            &surface,
            &adapter,
            &device,
            (size.width, size.height),
        );

        (event_loop, window, surface, device, queue, config)
    }

    pub fn init_view(window: &ProspectWindow) -> (SurfaceTexture, TextureView, CommandEncoder) {
        let (output, view) = GraphicsContext::create_view(window.get_surface());
        let command_encoder =
            GraphicsContext::create_command_encoder(window.get_device(), "Draw Loop Commands");
        (output, view, command_encoder)
    }

    pub fn start_render<'pass>(
        clear: (f64, f64, f64),
        view: &'pass TextureView,
        command_encoder: &'pass mut CommandEncoder,
    ) -> RenderPass<'pass> {
        let render_pass = GraphicsContext::begin_render_pass_barebones(
            (clear.0, clear.1, clear.2, 1.0),
            "Render Pass",
            view,
            command_encoder,
        );
        render_pass
    }

    pub fn finish_render(
        window: &ProspectWindow,
        command_encoder: CommandEncoder,
        output: SurfaceTexture,
    ) {
        window
            .get_queue()
            .submit(std::iter::once(command_encoder.finish()));
        output.present()
    }

    pub fn create_render_pipeline(name: &str, device : &Device, shader : &impl ProspectShader, bind_groups : Option<&Vec<&BindGroupLayout>>) -> RenderPipeline
    {
        let layout = GraphicsContext::create_pipeline_layout(name, device, bind_groups.unwrap_or(&vec![]));
        let pipeline = GraphicsContext::create_render_pipeline(name, &layout, shader.fragment_state(), shader.vertex_state(), device);
        pipeline
    }
}
