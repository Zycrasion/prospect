use vecto_rs::positional::Vector;
use wgpu::{Surface, Device, Queue, SurfaceConfiguration, Backends};
use winit::{event_loop::{EventLoop, ControlFlow}, window::{Window, WindowBuilder}, event::{self, WindowEvent, VirtualKeyCode, Event, KeyboardInput}, dpi::{Size, LogicalSize, PhysicalSize}};
use crate::prospect_app::*;
use crate::prospect_app::ProspectApp;

use super::{graphics_context::GraphicsContext, high_level_abstraction::HighLevelGraphicsContext};

pub struct ProspectWindow
{
    event_loop : Option<EventLoop<()>>,
    window : Window,
    surface : Surface,
    device : Device,
    queue : Queue,
    config : SurfaceConfiguration,
    pub size : (u32, u32),
}

impl ProspectWindow
{
    pub fn new<S : AsRef<str>>(title : S, width : u32, height : u32) -> Self
    {
        let (event_loop, window, surface, device, queue, config) = pollster::block_on(HighLevelGraphicsContext::init_window(title, width, height));
        
        Self
        {
            event_loop : Some(event_loop),
            window,
            surface,
            device,
            queue,
            config,
            size : (width, height)
        }
    }

    pub fn get_surface(&self) -> &Surface
    {
        &self.surface
    }

    pub fn get_device(&self) -> &Device
    {
        &self.device
    }

    pub fn get_queue(&self) -> &Queue
    {
        &self.queue
    }

    fn process_input(&self, ev : ProspectEvent, app : &mut Box<dyn ProspectApp>) -> Option<ControlFlow>
    {
        let response = app.process(ev);

        match response
        {
            ProcessResponse::CloseApp => Some(ControlFlow::Exit),
            ProcessResponse::ProspectProcess => if ev == ProspectEvent::KeyboardInput(Some(VirtualKeyCode::Escape)) {Some(ControlFlow::Exit)} else {None},
            ProcessResponse::DontProcess => None,
        }
    }

    fn resize(&mut self, size : &PhysicalSize<u32>)
    {
        if size.width <= 0 || size.height <= 0
        {
            return;
        }

        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.size = (size.width, size.height);
    }

    fn reconfigure(&mut self)
    {
        self.surface.configure(&self.device, &self.config)
    }

    pub fn run_with_app(mut self, mut app : Box<dyn ProspectApp>)
    {
        let event_loop = self.event_loop.take();
        let event_loop = event_loop.unwrap();

        event_loop.run(move |event, _, control_flow| {
            match event
            {
                Event::RedrawRequested(window_id) => if window_id == self.window.id()
                {
                    let result = app.draw(&self);
                    match result
                    {
                        Ok(_) => {},
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(wgpu::SurfaceError::Lost) => self.reconfigure(),
                        Err(e) => eprintln!("{:#?}", e)
                    }
                },
                Event::MainEventsCleared =>
                {
                    self.window.request_redraw();
                }
                Event::WindowEvent {
                    ref event,
                    window_id
                } if window_id == self.window.id() => {
                    match event
                    {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::KeyboardInput { input, ..} => 
                        {
                            if let Some(flow) = self.process_input(ProspectEvent::KeyboardInput(input.virtual_keycode), &mut app)
                            {
                                *control_flow = flow;
                            }
                        },
                        WindowEvent::CursorMoved { position, .. } =>
                        {
                            if let Some(flow) = self.process_input(ProspectEvent::CursorMoveEvent(Vector::new2(position.x as f32, position.y as f32)), &mut app)
                            {
                                *control_flow = flow;
                            }
                        }
                        WindowEvent::Resized(size) =>
                        {
                            self.resize(size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>
                        {
                            self.resize(new_inner_size);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
    }
}