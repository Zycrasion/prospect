use wgpu::{
    Adapter, Backends, Device, Dx12Compiler, Instance, InstanceDescriptor, Limits, PowerPreference,
    Queue, Surface, Features, RequestDeviceError, DeviceDescriptor, SurfaceConfiguration, TextureUsages,
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
        adapter.request_device(
            &DeviceDescriptor {
                label: Some("Device"),
                features: Features::empty(),
                limits:  if cfg!(target_arch = "wasm32") {
                    Limits::downlevel_webgl2_defaults()
                } else
                {
                    Limits::default()
                },
            },
            None,
        ).await
    }

    pub fn config_surface_easy(surface: &Surface, adapter: &Adapter, device : &Device, size : (u32, u32)) -> SurfaceConfiguration
    {
        let surface_caps = surface.get_capabilities(adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        if size.0 == 0 || size.1 == 0
        {
            eprintln!("Error: Size given to configure surface is 0 in height or width. Must be more than 0");
            panic!()
        }

        let config = SurfaceConfiguration
        {
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
}
