use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Device, FragmentState, ShaderModule, Surface,
    SurfaceConfiguration, TextureFormat, VertexState, VertexBufferLayout, RenderPipeline, ShaderStages, TextureViewDimension, TextureSampleType, BindGroupLayout, BindGroup, Sampler, Texture, TextureView,
};

use crate::{abstraction::{high_level_abstraction::HighLevelGraphicsContext, shader::ProspectShader, vertex::Vertex, prospect_window::ProspectWindow, graphics_context::GraphicsContext}, utils::prospect_fs::read_file_panic};

#[derive(Debug)]
pub struct TexturedShaderTexture
{
    pub group : BindGroup
}

pub struct TexturedShader {
    module: ShaderModule,
    color_target_state: Vec<Option<ColorTargetState>>,
    bind_layout : BindGroupLayout,
    sampler : Sampler
}

impl ProspectShader for TexturedShader {
    fn get_name(&self) -> &str {
        "Textured Shader"
    }

    fn get_module(&self) -> &ShaderModule {
        &self.module
    }

    fn fragment_state(&self) -> FragmentState {
        FragmentState {
            module: &self.module,
            entry_point: "fs_main",
            targets: &self.color_target_state,
        }
    }

    fn vertex_state(&self) -> VertexState {
        VertexState {
            module: &self.module,
            entry_point: "vs_main",
            buffers: &[Vertex::VERTEX_BUFFER_LAYOUT],
        }
    }

    fn build_render_pipeline(&self, device : &Device) -> RenderPipeline {
        HighLevelGraphicsContext::create_render_pipeline("Textured Shader Render Pipeline", device, self, Some(&vec![&self.bind_layout]))
    }
}

impl TexturedShader {
    pub fn new(
        window : &ProspectWindow
    ) -> Self {
        let surface = window.get_surface_config();
        let device = window.get_device();
        let src = read_file_panic("src/shaders/textured_shader.wgsl");

        let sampler = GraphicsContext::create_sampler("Textured Shader Sampler", device, None, None);
        let entries = vec![
            GraphicsContext::create_bind_group_layout_entry(0, ShaderStages::FRAGMENT, GraphicsContext::create_texture_binding_type(false, TextureViewDimension::D2, TextureSampleType::Float { filterable: true })),
            GraphicsContext::create_bind_group_layout_entry(1, ShaderStages::FRAGMENT, GraphicsContext::create_sample_binding_type(wgpu::SamplerBindingType::Filtering))
        ];
        let bind_group_layout = GraphicsContext::create_bind_group_layout(device, "Textured Shader Bind Group", &entries);

        Self {
            sampler,
            bind_layout: bind_group_layout,
            module: GraphicsContext::load_shader("Textured Shader", src.as_ref(), device),
            color_target_state: vec![Some(ColorTargetState {
                format: surface.format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
        }
    }
    
    pub fn create_texture(&self, window : &ProspectWindow, texture : &TextureView, name : &str) -> (u32, BindGroup)
    {
        let view_resource = GraphicsContext::create_texture_view_resource(0, texture);
        let sampler_resource = GraphicsContext::create_sampler_resource(1, &self.sampler);
        (0, GraphicsContext::create_bind_group(window.get_device(), name, &self.bind_layout, &vec![view_resource, sampler_resource]))
    }
}