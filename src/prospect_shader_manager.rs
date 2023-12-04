use std::collections::HashMap;

use wgpu::{RenderPipeline, RenderPass, Device, BindGroup};

use crate::abstraction::shader::ProspectShader;

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub struct ProspectShaderIndex(String);

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub struct ProspectBindGroupIndex(String);

pub struct ProspectShaderManager
{
    pipelines : HashMap<ProspectShaderIndex, RenderPipeline>,
    bind_groups : HashMap<ProspectBindGroupIndex, BindGroup>
}

impl ProspectShaderManager
{
    pub fn new() -> Self
    {
        Self { pipelines: HashMap::new(), bind_groups : HashMap::new() }
    }

    pub fn add_shader(&mut self, shader : &impl ProspectShader, device : &Device) -> Option<ProspectShaderIndex>
    {
        eprintln!("TODO: CHANGE METHOD OF ADDING BINDGROUPS/SHADERS TO REGISTRY");
        let result = self.pipelines.insert(ProspectShaderIndex(shader.get_name().to_owned()), shader.build_render_pipeline(device));
        if result.is_some()
        {
            return None;
        }

        Some(ProspectShaderIndex(shader.get_name().to_owned()))
    }

    pub fn apply_render_pipeline<'a>(&'a self, key : &ProspectShaderIndex, render_pass : &mut RenderPass<'a>)
    {
        let pipeline = self.pipelines.get(key);
        if pipeline.is_some()
        {
            let pipeline = pipeline.unwrap();
            render_pass.set_pipeline(pipeline);
        }
    }

    pub fn add_bind_group<S : AsRef<str>>(&mut self, name : S,  bind_group : BindGroup) -> Option<ProspectBindGroupIndex>
    {
        eprintln!("TODO: CHANGE METHOD OF ADDING BINDGROUPS/SHADERS TO REGISTRY");
        let result = self.bind_groups.insert(ProspectBindGroupIndex(name.as_ref().to_string()), bind_group);
        if result.is_some()
        {
            return None;
        }

        Some(ProspectBindGroupIndex(name.as_ref().to_string()))
    }

    pub fn apply_bind_group<'a>(&'a self, render_pass : &mut RenderPass<'a> ,key : &ProspectBindGroupIndex, loc : u32, offsets : &[u32])
    {
        let bind_group = self.bind_groups.get(key);
        if bind_group.is_some()
        {
            let bind_group = bind_group.unwrap();
            render_pass.set_bind_group(loc, bind_group, offsets)
        }
    }
}