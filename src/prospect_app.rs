use crate::abstraction::graphics_context::GraphicsContext;

#[derive(Clone, Copy)]
pub struct PropsectEvent;

pub trait ProspectApp
{
    fn setup(&mut self);
    
    fn draw(&mut self);

    fn process(&mut self, event : PropsectEvent);
}