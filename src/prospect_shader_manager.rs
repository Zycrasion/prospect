use std::{sync::Arc, collections::HashMap};

use wgpu::{RenderPipeline, RenderPass, Device};

use crate::abstraction::shader::ProspectShader;

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub struct ProspectShaderIndex(String);

pub struct ProspectShaderManager
{
    pipelines : HashMap<ProspectShaderIndex, RenderPipeline>
}

impl ProspectShaderManager
{
    pub fn new() -> Self
    {
        Self { pipelines: HashMap::new() }
    }

    pub fn add_shader(&mut self, shader : &impl ProspectShader, device : &Device) -> Option<ProspectShaderIndex>
    {
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
}