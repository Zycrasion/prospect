use crate::{abstraction::{mesh::{Mesh, Meshable}, prospect_window::ProspectWindow, graphics_context::GraphicsContext, high_level_abstraction::HighLevelGraphicsContext, shader::ProspectShader}, prospect_transform::{Transform, TransformUniform}, prospect_camera::ProspectCamera, prospect_shader_manager::ProspectBindGroupIndex};
use wgpu::*;


pub struct Model3D
{
    pub transform : Transform,
    matrix_buffer : Buffer,
    bind_group : ProspectBindGroupIndex
}

impl Model3D
{
    pub fn new(shader : &impl ProspectShader, window : &mut ProspectWindow) -> Model3D
    {
        let matrix_buffer = GraphicsContext::create_buffer(window.get_device(), "Transform Buffer", &[TransformUniform::default()], BufferUsages::COPY_DST | BufferUsages::UNIFORM);
        let bind_group = HighLevelGraphicsContext::create_uniform_with_bind_group(window.get_device(), "Transform Uniform", &matrix_buffer, shader.get_model_matrix_bind_layout().expect("Shader doesn't support Model View Matrix"));
        let bind_group = window.auto_add_bind_group(bind_group);
        Model3D { transform: Transform::new(), bind_group, matrix_buffer }
    }

    pub fn draw<'a>(&self, render_pass : &mut RenderPass<'a>, window : &'a ProspectWindow, cam : &'a ProspectCamera, mesh : &'a impl Meshable)
    {
        let data = self.transform.generate_matrix();
        GraphicsContext::update_buffer(window.get_queue(), &self.matrix_buffer, 0, &[data]);
        window.shader_manager.apply_bind_group(render_pass, &self.bind_group, 3, &[]);
        mesh.draw(render_pass, window.get_shader_manager(), cam);
    }

    pub fn draw_custom_bind_index<'a>(&self, render_pass : &mut RenderPass<'a>, window : &'a ProspectWindow, cam : &'a ProspectCamera, mesh : &'a impl Meshable, index : u32)
    {
        let data = self.transform.generate_matrix();
        GraphicsContext::update_buffer(window.get_queue(), &self.matrix_buffer, 0, &[data]);
        window.shader_manager.apply_bind_group(render_pass, &self.bind_group, index, &[]);
        mesh.draw(render_pass, window.get_shader_manager(), cam);
    }
}