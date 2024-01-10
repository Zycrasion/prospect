use wgpu::{BindGroupLayout, ShaderStages, BufferUsages};

use crate::{prospect_transform::{Transform, TransformUniform}, abstraction::{graphics_context::GraphicsContext, prospect_window::ProspectWindow, high_level_abstraction::HighLevelGraphicsContext}};

use super::SmartBindGroup;

#[derive(Clone, Debug)]
pub struct ModelInformation
{
    bind_group : SmartBindGroup,
    pub transform : Transform    
}

impl ModelInformation
{
    pub fn create_layout(window : &ProspectWindow) -> BindGroupLayout
    {
        let entries = [
            // Model Matrix
            GraphicsContext::create_bind_group_layout_entry(0, ShaderStages::VERTEX, GraphicsContext::create_uniform_binding_type())
        ];

        GraphicsContext::create_bind_group_layout(window.get_device(), "ModelInformation", &entries)
    }

    pub fn from_transform(window : &ProspectWindow, transform : Transform) -> Self {Self::new(window, Some(transform))}

    pub fn create(window : &ProspectWindow) -> Self { Self::new(window, None) }

    pub fn new(window : &ProspectWindow, transform : Option<Transform>) -> Self
    {
        let matrix_buffer = GraphicsContext::create_buffer(window.get_device(), "ModelInformation Matrix Buffer", &[TransformUniform::default()], BufferUsages::COPY_DST | BufferUsages::UNIFORM);

        Self
        {
            bind_group: HighLevelGraphicsContext::create_uniform_from_buffers(window.get_device(), "ModelInformation Bind Group", &vec![matrix_buffer], &Self::create_layout(window)).into(),
            transform : transform.unwrap_or(Transform::new()),
        }
    }

    pub fn get_bind_group(&self) -> SmartBindGroup
    {
        self.bind_group.clone()
    }
}