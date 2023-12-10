use std::f32::consts::PI;

use vecto_rs::linear::{Vector, VectorTrait, Mat4};
use wgpu::{Device, BufferUsages, ShaderStages, BindGroup, Buffer, RenderPass, BindGroupLayout, Queue};

use crate::abstraction::{graphics_context::GraphicsContext, prospect_window::ProspectWindow};


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_array([
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
]);

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CamUniform
{
    mat : [f32; 4 * 4]
}

impl CamUniform {
    pub fn new() -> Self
    {
        Self
        {
            mat : Mat4::identity().get_contents()
        }
    }

    pub fn update_proj(&mut self, cam : &Mat4)
    {
        self.mat = cam.get_contents()
    }
}

pub struct ProspectCamera {
    pub eye: Vector,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
    uniform : CamUniform,
    buffer : Buffer,
    bind_group : BindGroup,
    layout : BindGroupLayout,
}

impl ProspectCamera {
    pub fn new(device : &Device) -> ProspectCamera {
        let uniform = CamUniform::new();
        let buffer = GraphicsContext::create_buffer(&device, "Camera View Uniform Buffer", &[uniform], BufferUsages::UNIFORM | BufferUsages::COPY_DST);
        let (bind_group, layout) = Self::create_uniform(&buffer, &device);

        ProspectCamera {
            eye: Vector::new3(0., 0., 0.),
            fov: 90.,
            znear: 0.1,
            zfar: 100.,
            uniform : CamUniform::new(),
            buffer,
            bind_group,
            layout
        }
    }

    pub fn process_frame(&mut self, queue : &Queue)
    {
        let projection = &self.generate_projection_matrix();
        self.uniform.update_proj(&projection.get_column_major());
        GraphicsContext::update_buffer(queue, &self.buffer, 0, &[self.uniform]);
    }

    pub fn get_layout(&self) -> &BindGroupLayout
    {
        &self.layout
    }

    pub fn bind<'a>(&'a self, render_pass : &mut RenderPass<'a>, binding : u32)
    {
        render_pass.set_bind_group(binding, &self.bind_group, &[]);
    }

    pub fn generate_projection_matrix(&self) -> Mat4
    {
        let view = Mat4::new_transform(self.eye);
        let projection = Mat4::new_perspective_matrix(480., 480., self.fov, self.znear, self.zfar);
        let cam_matrix = OPENGL_TO_WGPU_MATRIX * projection * view;
        cam_matrix
    }

    fn create_uniform(buffer : &Buffer, device : &Device) -> (BindGroup, BindGroupLayout)
    {
        let cam_bind_group_layout_entry = GraphicsContext::create_bind_group_layout_entry(0, ShaderStages::VERTEX, GraphicsContext::create_uniform_binding_type());
        let cam_bind_group_layout = GraphicsContext::create_bind_group_layout(&device, "Camera Bind Group Layout", &vec![cam_bind_group_layout_entry]);

        let cam_bind_group_entry = GraphicsContext::create_bind_group_entry(0, buffer.as_entire_binding());
        let cam_bind_group = GraphicsContext::create_bind_group(&device, "Camera Bind Group", &cam_bind_group_layout, &vec![cam_bind_group_entry]);
        (cam_bind_group, cam_bind_group_layout)
    }
}
