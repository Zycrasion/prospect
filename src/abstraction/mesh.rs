use bytemuck::NoUninit;
use wgpu::{Buffer, BufferUsages, RenderPass};

use super::{vertex::Vertex, graphics_context::GraphicsContext};

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

    pub fn draw<'life>(&'life self, render_pass : &mut RenderPass<'life>)
    {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertex_count, 0..1);
    }
}