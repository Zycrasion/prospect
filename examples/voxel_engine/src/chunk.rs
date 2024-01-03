use std::collections::HashMap;

use crate::{vec_extension::VecPushIndex, voxel_shader::VoxelShader};
use noise::{NoiseFn, Perlin};
use prospect::{
    abstraction::{
        mesh::{Mesh, Meshable},
        prospect_window::ProspectWindow,
        shader::ProspectShader,
        vertex::Vertex,
    },
    linear::{Vector, VectorTrait, vector3},
    model::Model3D,
    prospect_camera::ProspectCamera,
    prospect_light::ProspectPointLight,
    prospect_shader_manager::{ProspectBindGroupIndex, ProspectShaderIndex},
    prospect_shape::ProspectShape,
    wgpu::RenderPass,
};

pub const BLOCK_TYPES: &[(f32, f32, &str)] = &[
    (0., 0., "air"),
    (BLOCK_UV * 8., BLOCK_UV * 5., "dirt"),
    (BLOCK_UV * 3., BLOCK_UV * 5., "cobblestone"),
    (BLOCK_UV * 4., BLOCK_UV * 5., "mossy cobblestone"),
    (0., BLOCK_UV * 12., "iron"),
    (BLOCK_UV * 8., BLOCK_UV * 4., "diamond"),
    (BLOCK_UV * 8., BLOCK_UV * 4., "diamond"),
];
pub const BLOCK_UV: f32 = 1. / 32.;
pub const BLOCK_TYPES_SIZE: u8 = 6;

pub const CHUNK_LWH: u32 = 32;
pub const BLOCKS_PER_CHUNK: u32 = CHUNK_LWH * CHUNK_LWH * CHUNK_LWH;
pub const VOXEL_SIZE: f32 = 0.5;
pub const CHUNK_SIZE: f32 = VOXEL_SIZE as f32 * CHUNK_LWH as f32;

macro_rules! get_block {
    ($name:ident, $x:expr, $y:expr, $z:expr) => {
        $name[index_into_block_array($x, $y, $z)]
    };
}

fn index_into_block_array(x: i32, y: i32, z: i32) -> usize {
    (x + (y * CHUNK_LWH as i32) + (z * CHUNK_LWH as i32 * CHUNK_LWH as i32)) as usize
}

fn generate(x : f32, y : f32, z : f32, i : i32, j : i32, k : i32, noise : Perlin) -> u8
{
    let x = x * CHUNK_LWH as f32;
    let y = y * CHUNK_LWH as f32;
    let z = z * CHUNK_LWH as f32;
    let block = noise
    .get([
        (i as f64 + x as f64) / 10. + 0.5,
        (j as f64 + y as f64) / 10. + 0.5,
        (k as f64 + z as f64) / 10. + 0.5,
    ])
    .abs()
    * BLOCK_TYPES_SIZE as f64;

    block.floor() as u8
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct ChunkEntry
{
    x : i32,
    y : i32,
    z : i32
}

pub fn from_vector(vec : Vector) -> ChunkEntry
{
    return ChunkEntry
    {
        x : vec.x.floor() as i32,
        y : vec.y.floor() as i32,
        z : vec.z.floor() as i32
    }
}

pub fn to_vector(vec : ChunkEntry) -> Vector
{
    return vector3(vec.x as f32, vec.y as f32, vec.z as f32)
}

pub struct ChunkData {
    // blocks: Box<[u8; BLOCKS_PER_CHUNK as usize]>,
    vertices : Vec<Vertex>,
    indices : Vec<u32>,
    x : f32,
    y : f32, 
    z : f32,
    pub entry : ChunkEntry
}

impl ChunkData {
    pub fn new(x: f32, y: f32, z: f32, noise: Perlin) -> Self {
        let mut blocks = Box::new([0u8; BLOCKS_PER_CHUNK as usize]);

        for i in 0..CHUNK_LWH as i32 {
            for j in 0..CHUNK_LWH as i32 {
                for k in 0..CHUNK_LWH as i32 {
                    blocks[index_into_block_array(i, j, k)] = generate(x, y, z, i, j, k, noise);
                }
            }
        }

        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];

        for i in 0..CHUNK_LWH as i32 {
            for j in 0..CHUNK_LWH as i32 {
                for k in 0..CHUNK_LWH as i32 {
                    let self_block = get_block!(blocks, i, j, k);
                    if self_block != 0 {
                        // Building Faces

                        // Top
                        let block = if j + 1 == CHUNK_LWH as i32 {
                            generate(x, y, z, i, j + 1, k, noise)
                        } else {
                            get_block!(blocks, i, j + 1, k)
                        };
                        if block == 0 {
                            let x = i as f32 * VOXEL_SIZE;
                            let y = j as f32 * VOXEL_SIZE;
                            let z = k as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [0., 1., 0.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [0., 1., 0.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [0., 1., 0.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [0., 1., 0.],
                            });

                            indices.push(v3 as u32);
                            indices.push(v1 as u32);
                            indices.push(v2 as u32);

                            indices.push(v3 as u32);
                            indices.push(v2 as u32);
                            indices.push(v4 as u32);
                        }
                        // Bottom
                        let block = if j == 0 {
                            generate(x, y, z, i, j - 1, k, noise)
                        } else {
                            get_block!(blocks, i, j - 1, k)
                        };
                        if block == 0 {
                            let x = i as f32 * VOXEL_SIZE;
                            let y = j as f32 * VOXEL_SIZE;
                            let z = k as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x, y, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [0., -1., 0.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [0., -1., 0.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x, y, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [0., -1., 0.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [0., -1., 0.],
                            });

                            indices.push(v3 as u32);
                            indices.push(v2 as u32);
                            indices.push(v1 as u32);

                            indices.push(v3 as u32);
                            indices.push(v4 as u32);
                            indices.push(v2 as u32);
                        }

                        // Back
                        let block = if k == 0 {
                            generate(x, y, z, i, j, k - 1, noise)
                        } else {
                            get_block!(blocks, i, j, k - 1)
                        };
                        if block == 0 {
                            let x = i as f32 * VOXEL_SIZE;
                            let y = j as f32 * VOXEL_SIZE;
                            let z = k as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x, y, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [0., 0., -1.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [0., 0., -1.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [0., 0., -1.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [0., 0., -1.],
                            });

                            indices.push(v3 as u32);
                            indices.push(v1 as u32);
                            indices.push(v2 as u32);

                            indices.push(v3 as u32);
                            indices.push(v2 as u32);
                            indices.push(v4 as u32);
                        }
                        // Front
                        let block = if k == CHUNK_LWH as i32 - 1 {
                            generate(x, y, z, i, j, k + 1, noise)
                        } else {
                            get_block!(blocks, i, j, k + 1)
                        };
                        if block == 0 {
                            let x = i as f32 * VOXEL_SIZE;
                            let y = j as f32 * VOXEL_SIZE;
                            let z = k as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x, y, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [0., 0., 1.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [0., 0., 1.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [0., 0., 1.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [0., 0., 1.],
                            });

                            indices.push(v3 as u32);
                            indices.push(v2 as u32);
                            indices.push(v1 as u32);

                            indices.push(v3 as u32);
                            indices.push(v4 as u32);
                            indices.push(v2 as u32);
                        }

                        // Left
                        let block = if i == 0 {
                            generate(x, y, z, i - 1, j, k, noise)
                        } else {
                            get_block!(blocks, i - 1, j, k)
                        };
                        if block == 0 {
                            let x = i as f32 * VOXEL_SIZE;
                            let y = j as f32 * VOXEL_SIZE;
                            let z = k as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x, y, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [0., 0., -1.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x, y, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [0., 0., -1.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [0., 0., -1.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [0., 0., -1.],
                            });

                            indices.push(v3 as u32);
                            indices.push(v2 as u32);
                            indices.push(v1 as u32);

                            indices.push(v3 as u32);
                            indices.push(v4 as u32);
                            indices.push(v2 as u32);
                        }
                        // Right
                        let block = if i == CHUNK_LWH as i32 - 1 {
                            generate(x, y, z, i + 1, j, k, noise)
                        } else {
                            get_block!(blocks, i + 1, j, k)
                        };
                        if block == 0 {
                            let x = i as f32 * VOXEL_SIZE;
                            let y = j as f32 * VOXEL_SIZE;
                            let z = k as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [1., 0., 0.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [1., 0., 0.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1 + BLOCK_UV,
                                ],
                                normal: [1., 0., 0.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [
                                    BLOCK_TYPES[self_block as usize].0 + BLOCK_UV,
                                    BLOCK_TYPES[self_block as usize].1,
                                ],
                                normal: [1., 0., 0.],
                            });

                            indices.push(v3 as u32);
                            indices.push(v2 as u32);
                            indices.push(v1 as u32);

                            indices.push(v3 as u32);
                            indices.push(v4 as u32);
                            indices.push(v2 as u32);
                        }
                    }
                }
            }
        }

        ChunkData { x, y, z, vertices, indices, entry: from_vector(vector3(x, y, z)) }
    }
}

pub struct Chunk {
    mesh: Mesh,
    model: Model3D,
}

impl Chunk {
    /** Minimum Corner, not center */
    pub fn new(
        data : ChunkData,
        window: &mut ProspectWindow,
        shader_key: &ProspectShaderIndex,
        shader: &impl ProspectShader,
        light: &ProspectPointLight,
        texture: &ProspectBindGroupIndex,
    ) -> Self {
        // Mesh Builder
        let mut vertices: Vec<Vertex> = data.vertices;
        let mut indices: Vec<u32> = data.indices;



        let shape = ProspectShape {
            vertices,
            indices: Some(indices),
        };

        let mut mesh = Mesh::from_shape(&shape, window.get_device(), shader_key);
        mesh.set_bind_group(1, light.get_bind_index());
        mesh.set_bind_group(3, texture);

        let mut model = Model3D::new(shader, window);
        model.transform.position = Vector::new3(data.x * CHUNK_SIZE, data.y * CHUNK_SIZE, data.z * CHUNK_SIZE);

        Chunk {
            // blocks,
            mesh,
            model,
        }
    }

    pub fn dist_from(&self, eye: Vector) -> f32 {
        eye.dist(&self.model.transform.position)
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        window: &'a ProspectWindow,
        cam: &'a ProspectCamera,
    ) {
        if self.model.transform.position.dist(&cam.eye) > cam.zfar / 1.5 {
            return;
        }
        self.model
            .draw_custom_bind_index(render_pass, window, cam, &self.mesh, 2);
    }
}
