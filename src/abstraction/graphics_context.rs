use wgpu::{
    Adapter, Backends, Color, CommandEncoder, CommandEncoderDescriptor, Device, DeviceDescriptor,
    Dx12Compiler, Features, Instance, InstanceDescriptor, Limits, LoadOp, Operations,
    PipelineLayout, PipelineLayoutDescriptor, PowerPreference, Queue, RenderPass,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RequestDeviceError,
    ShaderModule, ShaderModuleDescriptor, Surface, SurfaceConfiguration, SurfaceTexture,
    TextureUsages, TextureView, TextureViewDescriptor, RenderPipelineDescriptor, VertexState, FragmentState, ColorTargetState, BlendState, ColorWrites, PrimitiveState, PrimitiveTopology, FrontFace, Face, PolygonMode, MultisampleState,
};
use winit::{
    dpi::{PhysicalSize, Size},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct GraphicsContext;

impl GraphicsContext {
    pub fn init() {
        env_logger::init();
    }

    pub fn create_window<S: AsRef<str>>(
        title: S,
        width: u32,
        height: u32,
    ) -> (EventLoop<()>, Window) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title.as_ref())
            .with_inner_size(Size::Physical(PhysicalSize::new(width, height)))
            .build(&event_loop)
            .expect("Window Creation Failed");

        (event_loop, window)
    }

    pub fn create_instance(backends: Backends, dx12_shader_compiler: Dx12Compiler) -> Instance {
        Instance::new(InstanceDescriptor {
            backends,
            dx12_shader_compiler,
        })
    }

    pub fn create_surface(
        window: &Window,
        instance: &Instance,
    ) -> Result<Surface, wgpu::CreateSurfaceError> {
        unsafe { instance.create_surface(window) }
    }

    pub async fn create_adapter(instance: &Instance, surface: &Surface) -> Option<wgpu::Adapter> {
        instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
    }

    pub async fn create_device(adapter: &Adapter) -> Result<(Device, Queue), RequestDeviceError> {
        adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Device"),
                    features: Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        Limits::downlevel_webgl2_defaults()
                    } else {
                        Limits::default()
                    },
                },
                None,
            )
            .await
    }

    pub fn config_surface_easy(
        surface: &Surface,
        adapter: &Adapter,
        device: &Device,
        size: (u32, u32),
    ) -> SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        if size.0 == 0 || size.1 == 0 {
            eprintln!("Error: Size given to configure surface is 0 in height or width. Must be more than 0");
            panic!()
        }

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(device, &config);

        config
    }

    pub fn create_view(surface: &Surface) -> (SurfaceTexture, TextureView) {
        let output = surface
            .get_current_texture()
            .expect("Unable to get render texture");
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());
        (output, view)
    }

    pub fn create_command_encoder(device: &Device, name: &str) -> CommandEncoder {
        device.create_command_encoder(&CommandEncoderDescriptor { label: Some(name) })
    }

    pub fn begin_render_pass_barebones<'pass>(
        clear_color: (f64, f64, f64, f64),
        label: &str,
        view: &'pass TextureView,
        command_encoder: &'pass mut CommandEncoder,
    ) -> RenderPass<'pass> {
        command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: clear_color.0,
                        g: clear_color.1,
                        b: clear_color.2,
                        a: clear_color.3,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        })
    }

    pub fn load_shader(name: &str, src: &str, device: &Device) -> ShaderModule {
        device.create_shader_module(ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::Wgsl(src.into()),
        })
    }

    pub fn create_pipeline_layout(name: &str, device: &Device) -> PipelineLayout {
        device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some(name),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        })
    }

    pub fn create_render_pipeline(
        name: &str,
        layout: &PipelineLayout,
        fragment_state : FragmentState,
        vertex_state : VertexState,
        device: &Device,
    ) -> RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor
        {
            label : Some(name),
            layout: Some(layout),
            vertex: vertex_state,
            fragment: Some(fragment_state),
            primitive: PrimitiveState
            {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil : None,
            multisample : MultisampleState
            {
                count : 1,
                mask : !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        })
    }
}
