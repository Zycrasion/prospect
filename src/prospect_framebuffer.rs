use wgpu::{TextureView, Texture, Sampler, Device};

use crate::{abstraction::{high_level_abstraction::HighLevelGraphicsContext, graphics_context::GraphicsContext}, prospect_texture::BindableTexture};

pub struct ProspectFramebuffer
{
    view : TextureView,
    texture : Texture,
    sampler : Sampler
}

impl ProspectFramebuffer
{
    pub fn new(device : &Device, width : u32, height : u32) -> Self
    {
        let (texture, view, sampler) = GraphicsContext::create_framebuffer(device, "Framebuffer", width, height);

        Self
        {
            texture,
            view,
            sampler
        }
    }

    pub fn new_depth(device : &Device, width : u32, height : u32) -> Self
    {
        let (texture, view, sampler) = GraphicsContext::create_framebuffer_depth(device, "Framebuffer", width, height);

        Self
        {
            texture,
            view,
            sampler
        }
    }
}

impl BindableTexture for ProspectFramebuffer
{
    fn get_texture_view(&self) -> &TextureView {
        &self.view
    }

    fn get_name(&self) -> String {
        "Framebuffer".to_string()
    }
}