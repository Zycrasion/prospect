use winit::event::VirtualKeyCode;

#[derive(Clone, Copy)]
pub struct PropsectEvent
{
    pub key : Option<VirtualKeyCode>
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

    fn process(&mut self, event : PropsectEvent) -> ProcessResponse
    {
        ProcessResponse::ProspectProcess
    }
}