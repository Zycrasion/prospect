use winit::{event_loop::{EventLoop, ControlFlow}, window::{Window, WindowBuilder}, event::{self, WindowEvent, VirtualKeyCode, Event}, dpi::{Size, LogicalSize, PhysicalSize}};
use crate::prospect_app::*;
use crate::prospect_app::ProspectApp;

use super::graphics_context::GraphicsContext;

pub struct ProspectWindow
{
    event_loop : EventLoop<()>,
    window : Window
}

impl ProspectWindow
{
    pub fn new<S : AsRef<str>>(title : S, width : u32, height : u32) -> Self
    {
        let (event_loop, window) = GraphicsContext::create_window(title, width, height);
        Self
        {
            event_loop,
            window
        }
    }

    pub fn run_with_app(self, mut app : Box<dyn ProspectApp>)
    {
        let (event_loop, window) = (self.event_loop, self.window);

        event_loop.run(move |event, _, control_flow| {
            match event
            {
                Event::WindowEvent {
                    ref event,
                    window_id
                } if window_id == window.id() => {
                    match event
                    {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::KeyboardInput { input, ..} => 
                        {
                            let response = app.process(PropsectEvent {key : input.virtual_keycode});
                            
                            if response == ProcessResponse::CloseApp
                            {
                                *control_flow = ControlFlow::Exit;
                                return;
                            }

                            if response == ProcessResponse::DontProcess
                            {
                                return;
                            }

                            if input.virtual_keycode == Some(VirtualKeyCode::Escape)
                            {
                                *control_flow = ControlFlow::Exit;
                                return;
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