use log::{warn, error};
use wgpu::{RenderPipeline, RenderPass};

use super::{SmartBindGroup, SmartRenderPipeline};

#[derive(Clone, Debug)]
struct SmartBinding
{
    binding : SmartBindGroup,
    loc : u32
}


#[derive(Debug, Clone)]
pub struct SmartShader
{
    pipeline: SmartRenderPipeline,
    bindings : Vec<SmartBinding>,
    num_of_bindings : usize,
}

impl SmartShader
{
    pub fn new(pipeline : SmartRenderPipeline, binding_count : usize, groups : Vec<SmartBindGroup>) -> Self
    {
        if groups.len() != binding_count
        {
            error!("[ERROR]({}) SmartShader::new(...), binding count did not match length of bind groups recieved", file!());
        }

        let mut i = 0;
        let bindings : Vec<SmartBinding> = groups.iter().map(|value| SmartBinding {loc : {i += 1; i - 1}, binding : value.clone() }).collect();

        Self
        {
            pipeline,
            bindings,
            num_of_bindings : binding_count
        }
    }

    #[cfg(debug_assertions)]
    pub fn shader_is_valid(&self) -> bool
    {
        if self.bindings.len() == self.num_of_bindings
        {
            return true;
        }

        warn!("[WARN]({}) SmartShader Instance is not valid!", file!());
        return false;
    }

    #[cfg(not(debug_assertions))]
    pub fn shader_is_valid(&self) -> bool
    {
        self.bindings.len() == self.num_of_bindings
    }

    pub fn set_binding(&mut self, loc : u32, binding : SmartBindGroup) -> Result<(), ()>
    {
        if !self.shader_is_valid() {return Err(())}

        self.bindings.insert(loc as usize, SmartBinding { binding, loc });

        Ok(())
    }

    pub fn set_bindings<'a>(&'a self, render_pass : &mut RenderPass<'a>)
    {
        self.shader_is_valid(); // Since it doesn't matter that much here we just want the warn log

        self.bindings.iter().for_each(|binding| {
            binding.binding.set_bind_group(render_pass, binding.loc, &[]);
        });
    }

    pub fn set_shader<'a>(&'a self, render_pass : &mut RenderPass<'a>)
    {
        if !self.shader_is_valid()
        {
            return;
        }

        self.pipeline.apply(render_pass);
    }

    pub fn apply<'a>(&'a self, render_pass : &mut RenderPass<'a>)
    {
        if !self.shader_is_valid()
        {
            return;
        }

        self.set_bindings(render_pass);
        self.set_shader(render_pass);       
    }
}