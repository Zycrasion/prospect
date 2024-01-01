use crate::{vec_extension::VecPushIndex, voxel_shader::VoxelShader};
use noise::{NoiseFn, Perlin};
use prospect::{
    abstraction::{
        mesh::{Mesh, Meshable},
        prospect_window::ProspectWindow,
        shader::ProspectShader,
        vertex::Vertex,
    },
    model::Model3D,
    prospect_camera::ProspectCamera,
    prospect_light::ProspectPointLight,
    prospect_shader_manager::ProspectShaderIndex,
    prospect_shape::ProspectShape,
    wgpu::RenderPass, linear::{Vector, VectorTrait},
};

pub const CHUNK_LWH: u32 = 16;
pub const BLOCKS_PER_CHUNK: u32 = CHUNK_LWH * CHUNK_LWH * CHUNK_LWH;
pub const VOXEL_SIZE: f32 = 0.5;
pub const CHUNK_SIZE: f32 = VOXEL_SIZE as f32 * CHUNK_LWH as f32;

macro_rules! get_block {
    ($name:ident, $x:expr, $y:expr, $z:expr) => {
        $name[index_into_block_array($x, $y, $z)]
    };
}

fn index_into_block_array(x: u32, y: u32, z: u32) -> usize {
    (x + (y * CHUNK_LWH) + (z * CHUNK_LWH * CHUNK_LWH)) as usize
}

pub struct Chunk {
    mesh: Mesh,
    model: Model3D,
    blocks: Box<[u8; BLOCKS_PER_CHUNK as usize]>,
}

impl Chunk {
    /** Minimum Corner, not center */
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        noise: Perlin,
        window: &mut ProspectWindow,
        shader_key: &ProspectShaderIndex,
        shader: &impl ProspectShader,
        light: &ProspectPointLight,
    ) -> Self {
        let mut blocks = Box::new([0u8; BLOCKS_PER_CHUNK as usize]);

        for i in 0..CHUNK_LWH {
            for j in 0..CHUNK_LWH {
                for k in 0..CHUNK_LWH {
                    let block = noise.get([
                        i as f64 + x as f64 + 0.5,
                        j as f64 + y as f64 + 0.5,
                        k as f64 + z as f64 + 0.5,
                    ]);

                    blocks[index_into_block_array(i, j, k)] = if block > 0.5 { 1 } else { 0 };
                }
            }
        }

        blocks[index_into_block_array(5, 5, 5)] = 1;

        // Mesh Builder
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];

        for x in 0..CHUNK_LWH {
            for y in 0..CHUNK_LWH {
                for z in 0..CHUNK_LWH {
                    let block = get_block!(blocks, x, y, z);
                    if block == 1 {
                        // Building Faces

                        // Top
                        let block = if y + 1 == CHUNK_LWH {
                            0
                        } else {
                            get_block!(blocks, x, y + 1, z)
                        };
                        if block == 0 {
                            let x = x as f32 * VOXEL_SIZE;
                            let y = y as f32 * VOXEL_SIZE;
                            let z = z as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z],
                                uv: [0., 0.],
                                normal: [0., 1., 0.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z],
                                uv: [0., 1.],
                                normal: [0., 1., 0.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [0., 0.],
                                normal: [0., 1., 0.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [0., 0.],
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
                        let block = if y == 0 {
                            0
                        } else {
                            get_block!(blocks, x, y - 1, z)
                        };
                        if block == 0 {
                            let x = x as f32 * VOXEL_SIZE;
                            let y = y as f32 * VOXEL_SIZE;
                            let z = z as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x, y, z],
                                uv: [0., 0.],
                                normal: [0., -1., 0.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z],
                                uv: [0., 1.],
                                normal: [0., -1., 0.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x, y, z + VOXEL_SIZE],
                                uv: [0., 0.],
                                normal: [0., -1., 0.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z + VOXEL_SIZE],
                                uv: [0., 0.],
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
                        let block = if z == 0 {
                            0
                        } else {
                            get_block!(blocks, x, y, z - 1)
                        };
                        if block == 0 {
                            let x = x as f32 * VOXEL_SIZE;
                            let y = y as f32 * VOXEL_SIZE;
                            let z = z as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x, y, z],
                                uv: [0., 0.],
                                normal: [0., 0., -1.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z],
                                uv: [0., 1.],
                                normal: [0., 0., -1.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z],
                                uv: [0., 0.],
                                normal: [0., 0., -1.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z],
                                uv: [0., 0.],
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
                        let block = if z == CHUNK_LWH - 1 {
                            0
                        } else {
                            get_block!(blocks, x, y, z + 1)
                        };
                        if block == 0 {
                            let x = x as f32 * VOXEL_SIZE;
                            let y = y as f32 * VOXEL_SIZE;
                            let z = z as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x, y, z + VOXEL_SIZE],
                                uv: [0., 0.],
                                normal: [0., 0., 1.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z + VOXEL_SIZE],
                                uv: [0., 1.],
                                normal: [0., 0., 1.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [0., 0.],
                                normal: [0., 0., 1.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [0., 0.],
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
                        let block = if x == 0 {
                            0
                        } else {
                            get_block!(blocks, x - 1, y, z)
                        };
                        if block == 0 {
                            let x = x as f32 * VOXEL_SIZE;
                            let y = y as f32 * VOXEL_SIZE;
                            let z = z as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x, y, z],
                                uv: [0., 0.],
                                normal: [0., 0., -1.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x, y, z + VOXEL_SIZE],
                                uv: [0., 1.],
                                normal: [0., 0., -1.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z],
                                uv: [0., 0.],
                                normal: [0., 0., -1.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [0., 0.],
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
                        let block = if x == CHUNK_LWH - 1 {
                            0
                        } else {
                            get_block!(blocks, x + 1, y, z)
                        };
                        if block == 0 {
                            let x = x as f32 * VOXEL_SIZE;
                            let y = y as f32 * VOXEL_SIZE;
                            let z = z as f32 * VOXEL_SIZE;
                            let v1 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z],
                                uv: [0., 0.],
                                normal: [1., 0., 0.],
                            });
                            let v2 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z],
                                uv: [0., 1.],
                                normal: [1., 0., 0.],
                            });
                            let v3 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y, z + VOXEL_SIZE],
                                uv: [0., 0.],
                                normal: [1., 0., 0.],
                            });
                            let v4 = vertices.pushi(Vertex {
                                position: [x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE],
                                uv: [0., 0.],
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

        let shape = ProspectShape {
            vertices,
            indices: Some(indices),
        };

        let mut mesh = Mesh::from_shape(&shape, window.get_device(), shader_key);
        mesh.set_bind_group(1, light.get_bind_index());

        let mut model = Model3D::new(shader, window);
        model.transform.position = Vector::new3(x, y, z);

        Chunk {
            blocks,
            mesh,
            model,
        }
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        window: &'a ProspectWindow,
        cam: &'a ProspectCamera,
    ) {
        self.model
            .draw_custom_bind_index(render_pass, window, cam, &self.mesh, 2);
    }
}