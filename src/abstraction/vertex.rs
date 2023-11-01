use bytemuck::{Pod, Zeroable};
use vecto_rs::positional::Vector;
use wgpu::{VertexBufferLayout, VertexStepMode, VertexAttribute, VertexFormat, BufferAddress};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex
{
    pub position : [f32; 3],
    pub colour : [f32; 3]
}

impl Vertex
{
    pub const VERTEX_BUFFER_LAYOUT : VertexBufferLayout<'_> =         VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                shader_location: 1,
            },
        ],
    };


    pub fn create_vertex_buffer_layout<'lifetime>() -> VertexBufferLayout<'lifetime> {
        Self::VERTEX_BUFFER_LAYOUT
    }
}