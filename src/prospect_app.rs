use vecto_rs::linear::Vector;
use winit::{event::{VirtualKeyCode, ElementState}, dpi::PhysicalSize};

use crate::abstraction::prospect_window::ProspectWindow;

#[derive(Clone, Copy, PartialEq)]
pub enum ProspectEvent
{
    KeyboardInput(Option<VirtualKeyCode>, ElementState),
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