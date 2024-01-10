use vecto_rs::{linear::*, trig::*};




#[repr(C)]
#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug, Default)]
pub struct TransformUniform
{
    model_matrix : [f32; 4 * 4],
}

#[derive(Debug, Clone, Copy)]
pub struct Transform
{
    pub position : Vector,
    pub rotation : Vector,
    /// Scale is f32 because the engine doesn't support non-uniform scaling
    pub scale : f32
}

impl Transform
{
    pub fn new() -> Self
    {
        Self
        {
            position : Vector::new3(0., 0., 0.),
            rotation : Vector::new3(0., 0., 0.),
            scale : 1.
        }
    }

    pub fn generate_matrix(&self) -> TransformUniform
    {
        let mut matrix = Mat4::identity();
        matrix.scale(Vector4::new4(self.scale, self.scale, self.scale, 1.));
        matrix.rotate(to_radians((self.rotation.x / self.scale) % 360.), Vector::new3(1., 0., 0.));
        matrix.rotate(to_radians((self.rotation.y / self.scale) % 360.), Vector::new3(0., 1., 0.));
        matrix.rotate(to_radians((self.rotation.z / self.scale) % 360.), Vector::new3(0., 0., 1.));
        matrix.translate(self.position / self.scale);

        TransformUniform { model_matrix: matrix.transpose().get_contents()}
    }
}