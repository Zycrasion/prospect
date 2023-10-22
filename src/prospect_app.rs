use vecto_rs::positional::Vector;
use winit::{event::VirtualKeyCode, dpi::PhysicalSize};

use crate::abstraction::prospect_window::ProspectWindow;

#[derive(Clone, Copy, PartialEq)]
pub enum ProspectEvent
{
    KeyboardInput(Option<VirtualKeyCode>),
    CursorMoveEvent(Vector)
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