use std::time::{SystemTime, Duration};

use crate::prospect_camera::ProspectCamera;

use crate::{prospect_app::ProspectApp, prospect_shader_manager::ProspectBindGroupIndex};
use crate::prospect_shader_manager::{ProspectShaderManager, ProspectShaderIndex};
use crate::prospect_app::*;
use vecto_rs::linear::{Vector, VectorTrait};
use wgpu::{
   *
};
use winit::error::ExternalError;
use winit::window::CursorGrabMode;
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use super::graphics_context::GraphicsContext;
use super::{
    high_level_abstraction::HighLevelGraphicsContext,
    shader::ProspectShader,
};

pub struct ProspectWindow {
    event_loop: Option<EventLoop<()>>,
    window: Window,
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    depth_texture: (Texture, TextureView, Sampler),
    pub shader_manager: ProspectShaderManager,
    pub size: (u32, u32),
    delta : f64,
    last_frame : SystemTime,
}

 impl ProspectWindow {
    pub fn new<S: AsRef<str>>(
        title: S,
        width: u32,
        height: u32,
    ) -> Self {
        let (event_loop, window, surface, device, queue, config) =
            pollster::block_on(HighLevelGraphicsContext::init_window(title, width, height));

        let shader_manager = ProspectShaderManager::new();

        let depth_texture = GraphicsContext::create_depth_texture(&device, &config, "Depth Texture");

        Self {
            event_loop: Some(event_loop),
            window,
            surface,
            device,
            queue,
            config,
            size: (width, height),
            shader_manager,
            depth_texture,
            delta : 0.,
            last_frame : SystemTime::now()
        }
    }

    pub fn get_delta(&self) -> f64
    {
        self.delta
    }

    pub fn get_deltaf32(&self) -> f32
    {
        self.delta as f32
    }

    pub fn get_deltaf64(&self) -> f64
    {
        self.delta
    }

    // pub fn bind_groups(&mut self) -> Vec<&BindGroupLayout>
    // {
    //     vec![self.cam_bind_layout()]
    // }

    // pub fn cam_bind_layout(&mut self) -> &BindGroupLayout
    // {
    //     self.camera.get_layout().to_owned()
    // }
    
    pub fn get_depth_buffer(&self) -> &TextureView
    {
        &self.depth_texture.1
    }

    pub fn get_shader_manager(&self) -> &ProspectShaderManager
    {
        &self.shader_manager
    }

    pub fn get_surface_config(&self) -> &SurfaceConfiguration
    {
        &self.config
    }

    pub fn get_surface(&self) -> &Surface {
        &self.surface
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }

    pub fn get_queue(&self) -> &Queue {
        &self.queue
    }

    pub fn add_shader(&mut self, shader : &impl ProspectShader, camera : &ProspectCamera, uniforms : Vec<&BindGroupLayout>) -> ProspectShaderIndex
    {
        let mut a = uniforms.clone();
        a.insert(0, camera.get_layout());

        self.shader_manager.add_shader(shader, &self.device, a)
    }

    pub fn add_bind_group<S : AsRef<str>>(&mut self, name : S,  bind_group : BindGroup) -> ProspectBindGroupIndex
    {
        self.shader_manager.add_bind_group(name, bind_group)
    }

    pub fn auto_add_bind_group(&mut self, bind_group : BindGroup) -> ProspectBindGroupIndex
    {
        self.shader_manager.auto_add_bind_group(bind_group)
    }

    pub fn get_window(&mut self) -> &mut Window
    {
        &mut self.window
    }

    pub fn lock_cursor(&mut self,lock_mode : CursorGrabMode) -> Result<(), ExternalError>
    {
        self.window.set_cursor_grab(lock_mode)
    }

    fn process_input(
        &mut self,
        ev: ProspectEvent,
        app: &mut Box<dyn ProspectApp>,
    ) -> Option<ControlFlow> {
        let response = app.process(ev, self);

        match response {
            ProcessResponse::CloseApp => Some(ControlFlow::Exit),
            ProcessResponse::ProspectProcess => {
                if ev == ProspectEvent::KeyboardInput(Some(VirtualKeyCode::Escape), ElementState::Pressed) {
                    Some(ControlFlow::Exit)
                } else {
                    None
                }
            }
            ProcessResponse::DontProcess => None,
        }
    }


    fn resize(&mut self, size: &PhysicalSize<u32>) {
        if size.width <= 0 || size.height <= 0 {
            return;
        }

        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.size = (size.width, size.height);

        self.depth_texture = GraphicsContext::create_depth_texture(&self.device, &self.config, "Depth Texture");
    }

    fn reconfigure(&mut self) {
        self.surface.configure(&self.device, &self.config)
    }

    pub fn run_with_app(mut self, mut app: Box<dyn ProspectApp>) {
        let event_loop = self.event_loop.take();
        let event_loop = event_loop.unwrap();
        app.setup(&mut self);

        self.last_frame = SystemTime::now();
        event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(window_id) => {
                if window_id == self.window.id() {
                    self.delta = SystemTime::now().duration_since(self.last_frame).ok().unwrap_or(Duration::from_secs_f32(1. / 60.)).as_secs_f64();
                    self.last_frame = SystemTime::now();
                    let result = app.draw(&mut self);
                    match result {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(wgpu::SurfaceError::Lost) => self.reconfigure(),
                        Err(e) => eprintln!("{:#?}", e),
                    }
                }
            }
            Event::DeviceEvent { device_id: _, event } =>
            {
                match event
                {
                    winit::event::DeviceEvent::MouseMotion { delta } => {
                        if let Some(flow) = self.process_input(
                            ProspectEvent::CursorDelta(Vector::new2(delta.0 as f32, delta.1 as f32)),
                            &mut app,
                        ) {
                            *control_flow = flow;
                        }
                    },
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(flow) = self.process_input(
                        ProspectEvent::KeyboardInput(input.virtual_keycode, input.state),
                        &mut app,
                    ) {
                        *control_flow = flow;
                    }
                }
                WindowEvent::Focused(focused) => {
                    if let Some(flow) = self.process_input(
                        ProspectEvent::Focused(*focused),
                        &mut app,
                    ) {
                        *control_flow = flow;
                    }
                }
                WindowEvent::MouseInput { device_id: _, state, button, .. } =>
                {
                    if let Some(flow) = self.process_input(
                        ProspectEvent::CursorClicked(*state, *button),
                        &mut app,
                    ) {
                        *control_flow = flow;
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if let Some(flow) = self.process_input(
                        ProspectEvent::CursorMoveEvent(Vector::new2(
                            position.x as f32,
                            position.y as f32,
                        )),
                        &mut app,
                    ) {
                        *control_flow = flow;
                    }
                }
                WindowEvent::Resized(size) => {
                    self.resize(size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.resize(new_inner_size);
                }
                _ => {}
            },
            _ => {}
        })
    }
}