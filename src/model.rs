use crate::{abstraction::{mesh::Meshable, prospect_window::ProspectWindow, graphics_context::GraphicsContext, high_level_abstraction::HighLevelGraphicsContext, shader::ProspectShader}, prospect_transform::{Transform, TransformUniform}, prospect_camera::ProspectCamera, smart::SmartBindGroup};
use wgpu::*;


pub struct Model3D
{
    pub transform : Transform,
    matrix_buffer : Buffer,
    bind_group : SmartBindGroup
}

impl Model3D
{
    pub fn new(shader : &impl ProspectShader, window : &mut ProspectWindow) -> Model3D
    {
        let matrix_buffer = GraphicsContext::create_buffer(window.get_device(), "Transform Buffer", &[TransformUniform::default()], BufferUsages::COPY_DST | BufferUsages::UNIFORM);
        let bind_group = HighLevelGraphicsContext::create_uniform_with_bind_group(window.get_device(), "Transform Uniform", &matrix_buffer, shader.get_model_matrix_bind_layout().expect("Shader doesn't support Model View Matrix"));
        Model3D { transform: Transform::new(), bind_group : bind_group.into(), matrix_buffer }
    }

    pub fn draw<'a>(&'a self, render_pass : &mut RenderPass<'a>, window : &'a ProspectWindow, cam : &'a ProspectCamera, mesh : &'a impl Meshable)
    {
        let data = self.transform.generate_matrix();
        GraphicsContext::update_buffer(window.get_queue(), &self.matrix_buffer, 0, &[data]);
        self.bind_group.set_bind_group(render_pass,  3, &[]);
        mesh.draw(render_pass,  cam);
    }

    pub fn draw_custom_bind_index<'a>(&'a self, render_pass : &mut RenderPass<'a>, window : &'a ProspectWindow, cam : &'a ProspectCamera, mesh : &'a impl Meshable, index : u32)
    {
        let data = self.transform.generate_matrix();
        GraphicsContext::update_buffer(window.get_queue(), &self.matrix_buffer, 0, &[data]);
        self.bind_group.set_bind_group(render_pass, index, &[]);
        mesh.draw(render_pass, cam);
    }
}