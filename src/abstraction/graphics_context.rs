use bytemuck::NoUninit;

use env_logger::filter::Filter;
use wgpu::*;

use wgpu::util::{BufferInitDescriptor, DeviceExt};
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

        let present_mode = if surface_caps.present_modes.contains(&PresentMode::Fifo) {
            PresentMode::Fifo
        } else {
            println!(
                "Unable to Find Fifo Present Mode, Falling Back to first present mode ({:#?})",
                surface_caps.present_modes[0]
            );
            surface_caps.present_modes[0]
        };

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: present_mode,
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
        depth_view: &'pass TextureView,
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
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(Operations { load: LoadOp::Clear(1.0), store: true }),
                stencil_ops: None,
            }),
        })
    }

    pub fn load_shader(name: &str, src: &str, device: &Device) -> ShaderModule {
        device.create_shader_module(ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::Wgsl(src.into()),
        })
    }

    pub fn create_pipeline_layout(
        name: &str,
        device: &Device,
        bind_group_layouts: &Vec<&BindGroupLayout>,
    ) -> PipelineLayout {
        device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some(name),
            bind_group_layouts: &bind_group_layouts,
            push_constant_ranges: &[],
        })
    }

    pub fn create_render_pipeline(
        name: &str,
        layout: &PipelineLayout,
        fragment_state: FragmentState,
        vertex_state: VertexState,
        device: &Device,
    ) -> RenderPipeline {
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(name),
            layout: Some(layout),
            vertex: vertex_state,
            fragment: Some(fragment_state),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: GraphicsContext::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }

    pub fn create_buffer<A: NoUninit>(
        device: &Device,
        name: &str,
        contents: &[A],
        usage: wgpu::BufferUsages,
    ) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some(name),
            contents: bytemuck::cast_slice(contents),
            usage,
        })
    }

    pub fn update_buffer<A: NoUninit>(queue: &Queue, buffer: &Buffer, offset: u64, data: &[A]) {
        queue.write_buffer(buffer, offset, bytemuck::cast_slice(data))
    }

    pub fn create_texture(label: &str, bytes: &[u8], device: &Device, queue: &Queue) -> Texture {
        let img = image::load_from_memory(bytes).unwrap();
        let raw = img.to_rgba8();
        let dimensions = raw.dimensions();

        let size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &raw,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        texture
    }

    pub fn create_texture_view(texture: &Texture) -> TextureView {
        texture.create_view(&TextureViewDescriptor::default())
    }

    pub fn create_sampler(
        label: &str,
        device: &Device,
        mag_filter: Option<FilterMode>,
        min_filter: Option<FilterMode>,
    ) -> Sampler {
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some(label),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: mag_filter.unwrap_or(FilterMode::Linear),
            min_filter: min_filter.unwrap_or(FilterMode::Nearest),
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });
        sampler
    }

    pub fn create_sampler_advanced(
        label: &str,
        device: &Device,
        mag_filter: Option<FilterMode>,
        min_filter: Option<FilterMode>,
        mipmap_filter: Option<FilterMode>,
        compare: Option<CompareFunction>,
        lod_min_clamp: Option<f32>,
        lod_max_clamp: Option<f32>,
    ) -> Sampler {
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some(label),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: mag_filter.unwrap_or(FilterMode::Linear),
            min_filter: min_filter.unwrap_or(FilterMode::Nearest),
            mipmap_filter: mipmap_filter.unwrap_or(FilterMode::Nearest),
            compare,
            lod_min_clamp: lod_min_clamp.unwrap_or(0.),
            lod_max_clamp: lod_max_clamp.unwrap_or(100.),
            ..Default::default()
        });
        sampler
    }

    pub fn create_texture_binding_type(
        multisampled: bool,
        view_dimension: TextureViewDimension,
        sample_type: TextureSampleType,
    ) -> BindingType {
        BindingType::Texture {
            sample_type,
            view_dimension,
            multisampled,
        }
    }

    pub fn create_sample_binding_type(sampler_binding_type: SamplerBindingType) -> BindingType {
        BindingType::Sampler(sampler_binding_type)
    }

    pub fn create_uniform_binding_type() -> BindingType {
        BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        }
    }

    pub fn create_bind_group_layout_entry(
        binding: u32,
        shader_stage: ShaderStages,
        ty: BindingType,
    ) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding,
            visibility: shader_stage,
            ty,
            count: None,
        }
    }

    pub fn create_bind_group_layout(
        device: &Device,
        label: &str,
        entries: &Vec<BindGroupLayoutEntry>,
    ) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some(label),
            entries: &entries,
        })
    }

    pub fn create_texture_view_resource(binding: u32, view: &TextureView) -> BindGroupEntry {
        BindGroupEntry {
            binding,
            resource: BindingResource::TextureView(view),
        }
    }

    pub fn create_sampler_resource(binding: u32, sampler: &Sampler) -> BindGroupEntry {
        BindGroupEntry {
            binding,
            resource: BindingResource::Sampler(sampler),
        }
    }

    pub fn create_bind_group(
        device: &Device,
        label: &str,
        bind_group_layout: &BindGroupLayout,
        entries: &Vec<BindGroupEntry>,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some(label),
            entries: &entries,
            layout: &bind_group_layout,
        })
    }

    pub fn create_bind_group_entry<'a>(
        binding: u32,
        resource: BindingResource<'a>,
    ) -> BindGroupEntry<'a> {
        BindGroupEntry { binding, resource }
    }

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(
        device: &Device,
        config: &SurfaceConfiguration,
        label: &str,
    ) -> (Texture, TextureView, Sampler) {
        let size = Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let desc = TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = GraphicsContext::create_texture_view(&texture);
        let sampler = GraphicsContext::create_sampler_advanced(
            label,
            device,
            Some(FilterMode::Linear),
            Some(FilterMode::Linear),
            Some(FilterMode::Nearest),
            Some(CompareFunction::LessEqual),
            None,
            None,
        );

        (texture, view, sampler)
    }
}
