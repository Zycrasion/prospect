use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Device, FragmentState, ShaderModule, VertexState, RenderPipeline, ShaderStages, TextureViewDimension, TextureSampleType, BindGroupLayout, BindGroup, Sampler, TextureView,
};

use crate::{abstraction::{shader::ProspectShader, vertex::Vertex, high_level_abstraction::HighLevelGraphicsContext, prospect_window::ProspectWindow, graphics_context::GraphicsContext}, utils::prospect_fs::read_file_panic, prospect_shader_manager::ProspectBindGroupIndex, prospect_light::{LightUniform, ProspectPointLight}};

pub struct Default3D {
    module: ShaderModule,
    color_target_state: Vec<Option<ColorTargetState>>,
    bind_layout : BindGroupLayout,
    sampler : Sampler,
    matrix_bind_group_layout : BindGroupLayout
}

impl ProspectShader for Default3D {
    fn get_name(&self) -> &str {
        "3D Shader"
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

    fn get_model_matrix_bind_layout(&self) -> Option<&BindGroupLayout> {
        Some(&self.matrix_bind_group_layout)
    }

    fn build_render_pipeline(&self, device: &Device, bind_groups : Vec<&BindGroupLayout>) -> RenderPipeline {
        let mut bind_groups = bind_groups;
        bind_groups.insert(1, &self.bind_layout);
        bind_groups.insert(3, &self.matrix_bind_group_layout);

        HighLevelGraphicsContext::create_render_pipeline("Default 3D Shader Render Pipeline", device, self, Some(&bind_groups))
    }
}

impl Default3D {
    pub fn new(
        window : &ProspectWindow
    ) -> Self {
        let surface = window.get_surface_config();
        let device = window.get_device();
        let src = read_file_panic("src/shaders/default_3d.wgsl");

        let sampler = GraphicsContext::create_sampler("Default3D Shader Sampler", device, None, None);
        let entries = vec![
            GraphicsContext::create_bind_group_layout_entry(0, ShaderStages::FRAGMENT, GraphicsContext::create_texture_binding_type(false, TextureViewDimension::D2, TextureSampleType::Float { filterable: true })),
            GraphicsContext::create_bind_group_layout_entry(1, ShaderStages::FRAGMENT, GraphicsContext::create_sample_binding_type(wgpu::SamplerBindingType::Filtering))
        ];
        let bind_group_layout = GraphicsContext::create_bind_group_layout(device, "Default3D Shader Bind Group", &entries);

        let matrix_bind_group_layout = vec![
            GraphicsContext::create_bind_group_layout_entry(0, ShaderStages::VERTEX, GraphicsContext::create_uniform_binding_type())
        ];
        let matrix_bind_group_layout = GraphicsContext::create_bind_group_layout(device, "Default3D Matrix Bind Layout", &matrix_bind_group_layout);

        Self {
            sampler,
            bind_layout: bind_group_layout,
            matrix_bind_group_layout,
            module: GraphicsContext::load_shader("Default3D Shader", src.as_ref(), device),
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
        (2, GraphicsContext::create_bind_group(window.get_device(), name, &self.bind_layout, &vec![view_resource, sampler_resource]))
    }

    pub fn register_texture(&self, name: &str, bytes : &[u8], window: &mut ProspectWindow) -> ProspectBindGroupIndex
    {
        let texture_view = HighLevelGraphicsContext::create_texture_from_file(name, bytes, window);
        let bind_group = self.create_texture(window, &texture_view, name);
        window.add_bind_group(name, bind_group.1)
    }
}
