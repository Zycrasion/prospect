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
            config
        }
    }

    fn process_input(&self, input : &KeyboardInput, app : &mut Box<dyn ProspectApp>) -> Option<ControlFlow>
    {
        let response = app.process(ProspectEvent::KeyboardInput(input.virtual_keycode));

        match response
        {
            ProcessResponse::CloseApp => Some(ControlFlow::Exit),
            ProcessResponse::ProspectProcess => if input.virtual_keycode == Some(VirtualKeyCode::Escape) {Some(ControlFlow::Exit)} else {None},
            ProcessResponse::DontProcess => None,
        }
    }

    pub fn run_with_app(mut self, mut app : Box<dyn ProspectApp>)
    {
        let event_loop = self.event_loop.take();
        let event_loop = event_loop.unwrap();

        event_loop.run(move |event, _, control_flow| {
            match event
            {
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
                            if let Some(flow) = self.process_input(input, &mut app)
                            {
                                *control_flow = flow;
                            }
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        })
    }
}