use wgpu::{
    BlendState, ColorTargetState, ColorWrites, Device, FragmentState, ShaderModule, Surface,
    SurfaceConfiguration, TextureFormat, VertexState,
};

use super::graphics_context::GraphicsContext;

pub trait ProspectShader {
    fn get_name(&self) -> &str;
    fn get_module(&self) -> &ShaderModule;
    fn fragment_state(&self) -> FragmentState;
    fn vertex_state(&self) -> VertexState;
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
            buffers: &[],
        }
    }
}

impl BasicShader {
    pub fn new<S: AsRef<str>>(
        name: S,
        vertex_entry: S,
        fragment_entry: S,
        src: S,
        surface: &SurfaceConfiguration,
        device: &Device,
    ) -> Self {
        Self {
            name: name.as_ref().to_string(),
            vertex_entry: vertex_entry.as_ref().to_string(),
            fragment_entry: fragment_entry.as_ref().to_string(),
            module: GraphicsContext::load_shader(name.as_ref(), src.as_ref(), device),
            color_target_state: vec![Some(ColorTargetState {
                format: surface.format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
        }
    }
}
