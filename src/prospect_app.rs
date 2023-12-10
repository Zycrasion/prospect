use vecto_rs::linear::Vector;
use winit::event::{VirtualKeyCode, ElementState};

use crate::{abstraction::prospect_window::ProspectWindow, prospect_camera::ProspectCamera};

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

    fn process(&mut self, _event : ProspectEvent) -> ProcessResponse
    {
        ProcessResponse::ProspectProcess
    }
}