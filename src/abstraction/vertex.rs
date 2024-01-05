use bytemuck::{Pod, Zeroable};

use vecto_rs::linear::Vector;
use wgpu::{VertexBufferLayout, VertexStepMode, VertexAttribute, VertexFormat, BufferAddress};

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex
{
    pub position : [f32; 3],
    pub uv : [f32; 2],
    pub normal : [f32; 3]
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
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 5]>() as BufferAddress,
                shader_location: 2,
            },
        ],
    };
}

pub fn vertpos(x : f32, y : f32, z : f32) -> Vertex
{
    Vertex
    {
        position : [x, y, z],
        uv : [0.; 2],
        normal : [0.; 3]
    }
}

pub fn vertposuv(x : f32, y : f32, z : f32, u : f32, v : f32) -> Vertex
{
    Vertex
    {
        position : [x, y, z],
        uv : [u, v],
        normal : [0.; 3]
    }
}

pub fn vert(x : f32, y : f32, z : f32, u : f32, v : f32, nx : f32, ny : f32, nz : f32) -> Vertex
{
    Vertex
    {
        position : [x, y, z],
        uv : [u, v],
        normal : [nx, ny, nz]
    }
}