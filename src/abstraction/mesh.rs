use std::{rc::Rc, fmt::Write};

use bytemuck::NoUninit;
use wgpu::{Buffer, BufferUsages, RenderPass, Device, IndexFormat, RenderPipeline};
use winit::window::Window;

use crate::prospect_shape::ProspectShape;
use crate::prospect_shader_manager::*;
use super::{vertex::Vertex, graphics_context::GraphicsContext, prospect_window::ProspectWindow};

pub trait Meshable
{
    fn draw<'life>(&'life self, render_pass : &mut RenderPass<'life>, shader_manager : &'life ProspectShaderManager);
}

#[derive(Debug)]
pub struct Mesh
{
    vertex_buffer : Buffer,
    index_buffer : Buffer,
    index_count : u32,
    render_pipeline : ProspectShaderIndex
}

impl Mesh
{
    pub fn from_shape<T, U>(shape : &ProspectShape<T, U>, device : &Device, pipeline : &ProspectShaderIndex) -> Self
        where   T : Into<Vec<Vertex>> + Clone,
                U : Into<Vec<u16>> + Clone
    {
        let vertices = shape.vertices.clone().into();
        assert!(vertices.len() != 0);

        let indices = if shape.indices.is_none()
        {
            // Auto Generate indices
            let mut v = vec![];
            let l = vertices.len() as u16;
            for i in 0..l
            {
                v.push(l - i - 1);
            }
            v
        } else
        {
            shape.indices.clone().unwrap().into()
        };

        Self::new(vertices, indices, device, pipeline)
    }

    pub fn new<T, U>(vertices : T, indices : U, device : &Device, pipeline : &ProspectShaderIndex) -> Self 
        where   T : Into<Vec<Vertex>>,
                U : Into<Vec<u16>>
    {
        let vertices = vertices.into();
        let indices = indices.into();
        let count = indices.len();

        let vertex_buffer = GraphicsContext::create_buffer(device, "Vertex Buffer: MeshIndexed", &vertices, BufferUsages::VERTEX);
        let index_buffer = GraphicsContext::create_buffer(device, "Index Buffer: MeshIndexed", &indices, BufferUsages::INDEX);

        Self
        {
            vertex_buffer,
            index_buffer,
            index_count: count as u32,
            render_pipeline : pipeline.clone()
        }
    }
}

impl Meshable for Mesh
{
    fn draw<'life>(&'life self, render_pass : &mut RenderPass<'life>, shader_manager : &'life ProspectShaderManager) {
        shader_manager.apply_render_pipeline(&self.render_pipeline, render_pass);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);  
        render_pass.draw_indexed(0..self.index_count, 0, 0..1); 
    }
}