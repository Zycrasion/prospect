use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Device, FragmentState, ShaderModule, VertexState, RenderPipeline, BindGroup, BindGroupLayout,
};

use super::{graphics_context::GraphicsContext, vertex::Vertex, high_level_abstraction::HighLevelGraphicsContext, prospect_window::ProspectWindow};

pub trait ProspectShader : Sized {
    fn get_name(&self) -> &str;
    fn get_module(&self) -> &ShaderModule;
    fn fragment_state(&self) -> FragmentState;
    fn vertex_state(&self) -> VertexState;

    fn build_render_pipeline(&self, device : &Device, bind_groups : Vec<&BindGroupLayout>) -> RenderPipeline
    {
        HighLevelGraphicsContext::create_render_pipeline(self.get_name(), device, self, Some(&bind_groups))
    }
}

pub struct BasicShader {
    name: String,
    vertex_entry: String,
    fragment_entry: String,
    module: ShaderModule,
    color_target_state: Vec<Option<ColorTargetState>>,
}

impl ProspectShader for BasicShader {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_module(&self) -> &ShaderModule {
        &self.module
    }

    fn fragment_state(&self) -> FragmentState {
        FragmentState {
            module: &self.module,
            entry_point: &self.fragment_entry,
            targets: &self.color_target_state,
        }
    }

    fn vertex_state(&self) -> VertexState {
        VertexState {
            module: &self.module,
            entry_point: &self.vertex_entry,
            buffers: &[Vertex::VERTEX_BUFFER_LAYOUT],
        }
    }
}

impl BasicShader {
    pub fn new(
        window : &ProspectWindow
    ) -> Self {
        let surface = window.get_surface_config();
        let device = window.get_device();
        let src = include_str!("../shaders/shader.wgsl");

        Self {
            name: "Basic Shader".to_owned(),
            vertex_entry: "vs_main".to_owned(),
            fragment_entry: "fs_main".to_owned(),
            module: GraphicsContext::load_shader("Basic Shader", src.as_ref(), device),
            color_target_state: vec![Some(ColorTargetState {
                format: surface.format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
        }
    }
}
