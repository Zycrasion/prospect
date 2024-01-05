use std::{sync::Mutex, rc::Rc};

use wgpu::*;

#[derive(Debug)]
pub struct SmartBindGroup
{
    inner : Rc<BindGroup>
}

impl SmartBindGroup
{
    pub fn new(bind_group : BindGroup) -> Self
    {
        Self
        {
            inner : Rc::new(bind_group)
        }
    }

    pub fn set_bind_group<'a>(&'a self, render_pass : &mut RenderPass<'a>, index : u32, offsets : &[DynamicOffset])
    {
        render_pass.set_bind_group(index, &self.inner, offsets)
    }

}

impl Clone for SmartBindGroup
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl From<BindGroup> for SmartBindGroup
{
    fn from(value: BindGroup) -> Self {
        Self::new(value)
    }
}