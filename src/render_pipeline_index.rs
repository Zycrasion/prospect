use std::{sync::Arc, collections::HashMap};

use wgpu::{RenderPipeline, RenderPass, Device};

use crate::abstraction::{shader::ProspectShader, prospect_window::ProspectWindow};

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub struct RenderPipelineKey(String);

pub struct RenderPipelineIndex
{
    pipelines : HashMap<RenderPipelineKey, RenderPipeline>
}

impl RenderPipelineIndex
{
    pub fn new() -> Self
    {
        Self { pipelines: HashMap::new() }
    }

    pub fn add_shader(&mut self, shader : &impl ProspectShader, window : &ProspectWindow) -> Option<RenderPipelineKey>
    {
        let result = self.pipelines.insert(RenderPipelineKey(shader.get_name().to_owned()), shader.build_render_pipeline(window.get_device()));
        if result.is_some()
        {
            return None;
        }

        Some(RenderPipelineKey(shader.get_name().to_owned()))
    }

    pub fn apply_render_pipeline<'a>(&'a self, key : &RenderPipelineKey, render_pass : &mut RenderPass<'a>)
    {
        let pipeline = self.pipelines.get(key);
        if pipeline.is_some()
        {
            let pipeline = pipeline.unwrap();
            render_pass.set_pipeline(pipeline);
        }
    }
}