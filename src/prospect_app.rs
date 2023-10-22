use winit::{event::VirtualKeyCode, dpi::PhysicalSize};

use crate::abstraction::prospect_window::ProspectWindow;

#[derive(Clone, Copy)]
pub enum ProspectEvent
{
    KeyboardInput(Option<VirtualKeyCode>)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProcessResponse
{
    CloseApp,
    ProspectProcess,
    DontProcess,
}

pub trait ProspectApp
{
    fn setup(&mut self);
    
    fn draw(&mut self, window : &ProspectWindow) -> Result<(), wgpu::SurfaceError>;

    fn process(&mut self, event : ProspectEvent) -> ProcessResponse
    {
        ProcessResponse::ProspectProcess
    }
}