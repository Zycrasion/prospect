use vecto_rs::linear::Vector;
use winit::event::{VirtualKeyCode, ElementState, MouseButton};

use crate::{abstraction::prospect_window::ProspectWindow};

#[derive(Clone, Copy, PartialEq)]
pub enum ProspectEvent
{
    KeyboardInput(Option<VirtualKeyCode>, ElementState),
    CursorMoveEvent(Vector),
    CursorDelta(Vector),
    Focused(bool),
    CursorClicked(ElementState, MouseButton)
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
    fn setup(&mut self, window : &mut ProspectWindow);
    
    fn draw(&mut self, window : &mut ProspectWindow) -> Result<(), wgpu::SurfaceError>;

    fn process(&mut self, _event : ProspectEvent, _window : &mut ProspectWindow) -> ProcessResponse
    {
        ProcessResponse::ProspectProcess
    }
}