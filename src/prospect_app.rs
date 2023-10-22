use winit::{event::VirtualKeyCode, dpi::PhysicalSize};

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
    
    fn draw(&mut self);

    fn process(&mut self, event : ProspectEvent) -> ProcessResponse
    {
        ProcessResponse::ProspectProcess
    }

    fn resize(&mut self, new_size : PhysicalSize<u32>)
    {
        todo!()
    }
}