use bytemuck::NoUninit;
use wgpu::{Buffer, BufferUsages, RenderPass, Device, IndexFormat};

use super::{vertex::Vertex, graphics_context::GraphicsContext};

pub trait Meshable
{
    fn draw<'life>(&'life self, render_pass : &mut RenderPass<'life>);
}

pub struct Mesh
{
    vertices : Vec<Vertex>,
    vertex_buffer : Buffer,
    vertex_count : u32
}

impl Mesh
{
    pub fn from_vertices<T>(vertices : T, device : &wgpu::Device) -> Self
        where T : Into<Vec<Vertex>>
    {
        let vertices = vertices.into();
        let vertex_count = vertices.len() as u32;
        let vertex_buffer = GraphicsContext::create_buffer(device, "Mesh Vertex List", &vertices, BufferUsages::VERTEX);

        Self
        {
            vertices,
            vertex_buffer,
            vertex_count 
        }
    }
}

impl Meshable for Mesh
{
    fn draw<'life>(&'life self, render_pass : &mut RenderPass<'life>)
    {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertex_count, 0..1);
    }
}

#[derive(Debug)]
pub struct MeshIndexed
{
    vertex_buffer : Buffer,
    index_buffer : Buffer,
    index_count : u32
}

impl MeshIndexed
{
    pub fn new<T, U>(vertices : T, indices : U, device : &Device) -> Self 
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
            index_count: count as u32
        }
    }
}

impl Meshable for MeshIndexed
{
    fn draw<'life>(&'life self, render_pass : &mut RenderPass<'life>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);  
        render_pass.draw_indexed(0..self.index_count, 0, 0..1); 
    }
}