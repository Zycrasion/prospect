
use vecto_rs::linear::*;
use wgpu::*;

use crate::{abstraction::{graphics_context::GraphicsContext, prospect_window::ProspectWindow, high_level_abstraction::HighLevelGraphicsContext}, prospect_shader_manager::ProspectBindGroupIndex};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct LightUniform {
    position: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding: u32,
    color: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding2: u32,
}

pub struct ProspectPointLight
{
    pub colour : Vector,
    pub position : Vector,
    index : ProspectBindGroupIndex,
    buffer : Buffer,
    layout : BindGroupLayout,
}

impl ProspectPointLight
{
    pub fn new(window : &mut ProspectWindow) -> Self    
    {
        let buffer = Self::build_buffer(window, LightUniform::default());
        let (layout, bind_group) = Self::build_uniform(window, &buffer);


        Self
        {
            colour : Vector::new3(1., 1., 1.),
            position : Vector::default(),
            index: window.auto_add_bind_group(bind_group),
            buffer,
            layout,
        }
    }

    pub fn copy_index(&self) -> ProspectBindGroupIndex
    {
        self.index.clone()
    }

    pub fn get_bind_index(&self) -> &ProspectBindGroupIndex
    {
        &self.index
    }

    pub fn process_frame(&self, window : &ProspectWindow)
    {
        let data = self.generate_data();
        GraphicsContext::update_buffer(window.get_queue(), &self.buffer, 0, &[data]);
    }

    pub fn get_layout(&self) -> &BindGroupLayout
    {
        &self.layout
    }

    fn build_uniform(window : &ProspectWindow, buffer : &Buffer) -> (BindGroupLayout, BindGroup)
    {
        HighLevelGraphicsContext::create_uniform_and_bind_group(window.get_device(), "Point Light Uniform", ShaderStages::VERTEX_FRAGMENT, &buffer)
    }

    fn build_buffer(window : &ProspectWindow, data : LightUniform) -> Buffer
    {
        let buffer = GraphicsContext::create_buffer(window.get_device(), "Light Uniform Buffer", &[data], BufferUsages::UNIFORM | BufferUsages::COPY_DST);
        buffer
    }

    fn generate_data(&self) -> LightUniform
    {
        LightUniform { position: [self.position.x, self.position.y, self.position.z], _padding: 0, color: [self.colour.x, self.colour.y, self.colour.z], _padding2: 0 }
    }
}