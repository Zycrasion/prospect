use noise::{Perlin, NoiseFn};
use prospect::{abstraction::vertex::Vertex, linear::{Vector, VectorTrait}, prospect_shape::ProspectShape};

fn generate_height(x_offset : f32, y_offset : f32, x : f32, y : f32, perlin : Perlin, z : f32) -> f32
{
    perlin.get([((x_offset + x) / 3.) as f64, ((y + y_offset) / 3.) as f64, z as f64]) as f32
}

fn generate_vertex(x_offset : f32, z_offset : f32, x : f32, z : f32, perlin : Perlin) -> Vertex
{
    let b = Vector::new3(x, generate_height(x_offset, z_offset, x, z, perlin, 0.), z);
    let a = Vector::new3(x + 0.1, generate_height(x_offset, z_offset, x + 0.1, z, perlin, 0.), z);
    let c = Vector::new3(x, generate_height(x_offset, z_offset, x, z + 0.1, perlin, 0.), z + 0.1);
    
    let ab = b - a;
    let bc = c - b;

    let normal = Vector::cross(&ab, &bc).normalized();

    let y = generate_height(x_offset, z_offset, x, z, perlin, 0.);
    let uv_x = 1. - (y / 2. + 0.5);
    let uv_y_vary = perlin.get([x as f64 / 10., z as f64 / 10., 100.]) as f32;
    let uv_y = 5. / 16. + uv_y_vary / 16.;

    Vertex
    {
        position : [x, y, z],
        uv : [uv_x, uv_y]  ,
        normal : [normal.x, normal.y, normal.z]
    }
}

pub struct Chunk;

impl Chunk
{
    pub fn generate(x_offset : f32, z_offset : f32) -> ProspectShape<Vec<Vertex>, Vec<u32>>
    {
        let mut shape : ProspectShape<Vec<Vertex>, Vec<u32>> = ProspectShape { vertices: vec![], indices: Some(vec![]) };

        let mut indices = vec![];
        let size = 50 + 1;
        let perlin = Perlin::new(0);

        for z in 0..size
        {
            for x in 0..size
            {
                shape.vertices.push(generate_vertex(x_offset, z_offset, (x as f32 - size as f32 / 2.) / 4., (z as f32 - size as f32 / 2.) as f32 / 4., perlin));
                if x + 1 < size && z + 1 < size
                {
                    indices.push(z * size + x + 1);
                    indices.push((z + 1) * size + x);
                    indices.push(z * size + x);

                    indices.push((z + 1) * size + x + 1);
                    indices.push((z + 1) * size + x);
                    indices.push(z * size + x + 1);
                }
            }
        }

        shape.indices = Some(indices);

        shape
    }
}