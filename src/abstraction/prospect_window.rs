use crate::prospect_app::ProspectApp;
use crate::prospect_app::*;
use vecto_rs::positional::Vector;
use wgpu::{
    Backends, Device, Queue, RenderPipeline, Surface, SurfaceConfiguration, VertexBufferLayout,
};
use winit::{
    dpi::{LogicalSize, PhysicalSize, Size},
    event::{self, Event, KeyboardInput, VirtualKeyCode, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use super::{
    graphics_context::GraphicsContext,
    high_level_abstraction::HighLevelGraphicsContext,
    shader::{BasicShader, ProspectShader},
};

pub struct ProspectWindow {
    event_loop: Option<EventLoop<()>>,
    window: Window,
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    pub size: (u32, u32),
}

impl ProspectWindow {
    pub fn new<S: AsRef<str>>(
        title: S,
        width: u32,
        height: u32,
    ) -> Self {
        let (event_loop, window, surface, device, queue, config) =
            pollster::block_on(HighLevelGraphicsContext::init_window(title, width, height));

        let main_shader = BasicShader::new(
            "Main Shader",
            "vs_main",
            "fs_main",
            include_str!("../shaders/shader.wgsl"),
            &config,
            &device,
        );

        let render_pipeline = HighLevelGraphicsContext::create_render_pipeline("Main Pipeline", &device, &main_shader);

        Self {
            event_loop: Some(event_loop),
            window,
            surface,
            device,
            queue,
            config,
            size: (width, height),
            render_pipeline,
        }
    }

    pub fn get_render_pipeline(&self) -> &RenderPipeline {
        &self.render_pipeline
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

    fn process_input(
        &self,
        ev: ProspectEvent,
        app: &mut Box<dyn ProspectApp>,
    ) -> Option<ControlFlow> {
        let response = app.process(ev);

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
    }

    fn reconfigure(&mut self) {
        self.surface.configure(&self.device, &self.config)
    }

    pub fn run_with_app(mut self, mut app: Box<dyn ProspectApp>) {
        let event_loop = self.event_loop.take();
        let event_loop = event_loop.unwrap();

        event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(window_id) => {
                if window_id == self.window.id() {
                    let result = app.draw(&self);
                    match result {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(wgpu::SurfaceError::Lost) => self.reconfigure(),
                        Err(e) => eprintln!("{:#?}", e),
                    }
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
