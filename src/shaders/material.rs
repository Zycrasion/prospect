use wgpu::RenderPass;

use super::textured_shader::TexturedShaderTexture;


#[derive(Default, Debug)]
pub enum Material
{
    #[default]
    BlankMaterial,
    TexturedMaterial(TexturedShaderTexture),
}

impl Material
{
    pub fn apply_to_render_pass<'life>(&'life self, render_pass : &mut RenderPass<'life>)
    {
        match self
        {
            Material::BlankMaterial => {},
            Material::TexturedMaterial(tex) => 
            {
                render_pass.set_bind_group(0, &tex.group, &[]);
            },
        }
    }
}