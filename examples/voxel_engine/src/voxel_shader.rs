use prospect::abstraction::graphics_context::GraphicsContext;
use prospect::abstraction::high_level_abstraction::HighLevelGraphicsContext;
use prospect::abstraction::prospect_window::ProspectWindow;
use prospect::abstraction::shader::ProspectShader;
use prospect::abstraction::vertex::Vertex;
use prospect::wgpu::{*, self};

pub struct VoxelShader
{
    module: ShaderModule,
    color_target_state: Vec<Option<ColorTargetState>>,
    matrix_bind_group_layout : BindGroupLayout,
}

impl ProspectShader for VoxelShader
{
    fn get_name(&self) -> &str {
        "VoxelShader"
    }

    fn get_module(&self) -> &prospect::wgpu::ShaderModule {
        todo!()
    }

    fn fragment_state(&self) -> prospect::wgpu::FragmentState {
        FragmentState {
            module: &self.module,
            entry_point: "fs_main",
            targets: &self.color_target_state,
        }
    }

    fn vertex_state(&self) -> prospect::wgpu::VertexState {
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
        bind_groups.insert(2, &self.matrix_bind_group_layout);

        HighLevelGraphicsContext::create_render_pipeline("Voxel Render Pipeline", device, self, Some(&bind_groups))
    }
}

impl VoxelShader
{

    pub fn new(
        window : &ProspectWindow,
    ) -> Self {
        let surface = window.get_surface_config();
        let device = window.get_device();
        let src = include_str!("shader/voxel_shader.wgsl");

        let matrix_bind_group_layout = vec![
            GraphicsContext::create_bind_group_layout_entry(0, ShaderStages::VERTEX, GraphicsContext::create_uniform_binding_type())
        ];
        let matrix_bind_group_layout = GraphicsContext::create_bind_group_layout(device, "Voxel Matrix Bind Layout", &matrix_bind_group_layout);

        Self {
            matrix_bind_group_layout,
            module: GraphicsContext::load_shader("Voxel Shader", src.as_ref(), device),
            color_target_state: vec![Some(ColorTargetState {
                format: surface.format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
        }
    }
}