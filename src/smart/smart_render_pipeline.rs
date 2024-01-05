use std::rc::Rc;

use wgpu::*;

#[derive(Debug)]
pub struct SmartRenderPipeline
{
    inner : Rc<RenderPipeline>
}

impl SmartRenderPipeline
{
    pub fn new(pipeline : RenderPipeline) -> Self
    {
        Self
        {
            inner : Rc::new(pipeline)
        }
    }

    pub fn apply<'a>(&'a self, pass : &mut RenderPass<'a>)
    {
        pass.set_pipeline(&self.inner)
    }
}

impl Clone for SmartRenderPipeline
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl From<RenderPipeline> for SmartRenderPipeline
{
    fn from(value: RenderPipeline) -> Self {
        Self::new(value)
    }
}