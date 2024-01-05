use std::collections::HashMap;



use wgpu::{Buffer, BufferUsages, RenderPass, Device, BindGroup};


use crate::smart::{SmartRenderPipeline, SmartBindGroup};
use crate::{prospect_shape::ProspectShape, prospect_camera::ProspectCamera};
use super::{vertex::Vertex, graphics_context::GraphicsContext};

pub trait Meshable
{
    fn draw<'life>(&'life self, render_pass : &mut RenderPass<'life>, cam : &'life ProspectCamera);
}

#[derive(Debug)]
pub struct Mesh
{
    vertex_buffer : Buffer,
    index_buffer : Buffer,
    index_count : u32,
    render_pipeline : SmartRenderPipeline,
    bind_groups : HashMap<u32, SmartBindGroup>
}

impl Mesh
{
    pub fn from_shape<T, U>(shape : &ProspectShape<T, U>, device : &Device, pipeline : &SmartRenderPipeline) -> Self
        where   T : Into<Vec<Vertex>> + Clone,
                U : Into<Vec<u32>> + Clone
    {
        let vertices = shape.vertices.clone().into();

        let indices = if shape.indices.is_none()
        {
            // Auto Generate indices
            let mut v = vec![];
            let l = vertices.len() as u32;
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

    pub fn new<T, U>(vertices : T, indices : U, device : &Device, pipeline : &SmartRenderPipeline) -> Self 
        where   T : Into<Vec<Vertex>>,
                U : Into<Vec<u32>>
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
            render_pipeline : pipeline.clone(),
            bind_groups : HashMap::new()
        }
    }

    pub fn set_bind_group(&mut self, loc : u32, bind_group : &SmartBindGroup)
    {
        self.bind_groups.insert(loc, bind_group.clone());
    }
}

impl Meshable for Mesh
{
    fn draw<'life>(&'life self, render_pass : &mut RenderPass<'life>, cam : &'life ProspectCamera) {
        self.render_pipeline.apply(render_pass);
        
        for bind_group in &self.bind_groups
        {
            bind_group.1.set_bind_group(render_pass, *bind_group.0, &[]);
        }

        cam.bind(render_pass, 0);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);  
        render_pass.draw_indexed(0..self.index_count, 0, 0..1); 
    }
}