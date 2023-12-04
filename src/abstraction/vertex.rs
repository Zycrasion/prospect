use bytemuck::{Pod, Zeroable};

use wgpu::{VertexBufferLayout, VertexStepMode, VertexAttribute, VertexFormat, BufferAddress};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex
{
    pub position : [f32; 3],
    pub uv : [f32; 2]
}

impl Vertex
{
    pub const VERTEX_BUFFER_LAYOUT : VertexBufferLayout<'static> =         VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                shader_location: 1,
            },
        ],
    };
}