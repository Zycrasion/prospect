use vecto_rs::{linear::*, trig::*};
use wgpu::*;

use crate::{prospect_shader_manager::ProspectBindGroupIndex, abstraction::{graphics_context::GraphicsContext, prospect_window::ProspectWindow, high_level_abstraction::HighLevelGraphicsContext}};

#[repr(C)]
#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug, Default)]
pub struct TransformUniform
{
    model_matrix : [f32; 4 * 4],
}

#[derive(Debug)]
pub struct Transform
{
    pub position : Vector,
    pub rotation : Vector,
}

impl Transform
{
    pub fn new() -> Self
    {
        Self
        {
            position : Vector::new3(0., 0., 0.),
            rotation : Vector::new3(0., 0., 0.),

        }
    }

    pub fn generate_matrix(&self) -> TransformUniform
    {
        let mut matrix = Mat4::identity();
        matrix.rotate(to_radians(self.rotation.x % 360.), Vector::new3(1., 0., 0.));
        matrix.rotate(to_radians(self.rotation.y % 360.), Vector::new3(0., 1., 0.));
        matrix.rotate(to_radians(self.rotation.z % 360.), Vector::new3(0., 0., 1.));
        matrix.translate(self.position);

        TransformUniform { model_matrix: matrix.transpose().get_contents()}
    }
}