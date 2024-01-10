use std::rc::Rc;

use wgpu::*;

use crate::{
    abstraction::{
        graphics_context::GraphicsContext, high_level_abstraction::HighLevelGraphicsContext,
        prospect_window::ProspectWindow, vertex::Vertex,
    },
    prospect_camera::ProspectCamera,
    prospect_light::ProspectPointLight,
    prospect_texture::ProspectTexture,
    prospect_transform::Transform,
    smart::*,
};

pub trait Shader3D: Sized {}

/// Smart Version Of Default3D
///
/// (Uses same shader code internally)
pub struct Smart3D {
    shader: SmartShader,
    sampler: Rc<Sampler>,
    bind_layout: Rc<BindGroupLayout>,
    model_info: ModelInformation,
}

impl Smart3D {
    pub fn new(
        window: &ProspectWindow,
        camera: &ProspectCamera,
        light: &ProspectPointLight,
        texture: &ProspectTexture,
    ) -> Self {
        let surface = window.get_surface_config();
        let device = window.get_device();
        let src = include_str!("default_3d.wgsl");
        let module = &GraphicsContext::load_shader("Smart3D", src, device);

        let frag_state = FragmentState {
            module,
            entry_point: "fs_main",
            targets: &[Some(ColorTargetState {
                format: surface.format,
                blend: Some(BlendState::REPLACE),
                write_mask: ColorWrites::ALL,
            })],
        };

        let vert_state = VertexState {
            module,
            entry_point: "vs_main",
            buffers: &[Vertex::VERTEX_BUFFER_LAYOUT],
        };

        let sampler = GraphicsContext::create_sampler("Smart3D Shader Sampler", device, None, None);
        let entries = vec![
            GraphicsContext::create_bind_group_layout_entry(
                0,
                ShaderStages::FRAGMENT,
                GraphicsContext::create_texture_binding_type(
                    false,
                    TextureViewDimension::D2,
                    TextureSampleType::Float { filterable: true },
                ),
            ),
            GraphicsContext::create_bind_group_layout_entry(
                1,
                ShaderStages::FRAGMENT,
                GraphicsContext::create_sample_binding_type(wgpu::SamplerBindingType::Filtering),
            ),
        ];
        let texture_bind_layout = GraphicsContext::create_bind_group_layout(
            device,
            "Smart3D Shader Bind Group",
            &entries,
        );

        let matrix_bind_group_layout = vec![GraphicsContext::create_bind_group_layout_entry(
            0,
            ShaderStages::VERTEX,
            GraphicsContext::create_uniform_binding_type(),
        )];
        let matrix_bind_group_layout = GraphicsContext::create_bind_group_layout(
            device,
            "Default3D Matrix Bind Layout",
            &matrix_bind_group_layout,
        );

        let pipeline = HighLevelGraphicsContext::create_render_pipeline2(
            "Smart3D",
            device,
            vert_state,
            frag_state,
            Some(&vec![
                camera.get_layout(),
                &texture_bind_layout,
                light.get_layout(),
                &matrix_bind_group_layout,
            ]),
        );

        let view_resource =
            GraphicsContext::create_texture_view_resource(0, texture.get_texture_view());
        let sampler_resource = GraphicsContext::create_sampler_resource(1, &sampler);
        let group = GraphicsContext::create_bind_group(
            window.get_device(),
            "Smart3D",
            &texture_bind_layout,
            &vec![view_resource, sampler_resource],
        )
        .into();

        let model_info = ModelInformation::create(window);

        let shader = SmartShader::new(pipeline.into(), 4, vec![camera.get_bind_group(), group, light.get_bind_group(), model_info.get_bind_group()]);

        Self {
            shader,
            sampler: Rc::new(sampler),
            bind_layout: Rc::new(texture_bind_layout),
            model_info,
        }
    }

    pub fn set_transform(&mut self, transform: &Transform) -> &Self {
        self.model_info.transform = *transform;
        self
    }

    pub fn bind_light(&mut self, light: &ProspectPointLight) -> &Self {
        self.shader.set_binding(2, light.get_bind_group()).unwrap();
        self
    }

    pub fn bind_texture(&mut self, texture: ProspectTexture, window: &mut ProspectWindow) -> &Self {
        let view_resource =
            GraphicsContext::create_texture_view_resource(0, texture.get_texture_view());
        let sampler_resource = GraphicsContext::create_sampler_resource(1, &self.sampler);
        self.shader
            .set_binding(
                1,
                GraphicsContext::create_bind_group(
                    window.get_device(),
                    "Smart3D",
                    &self.bind_layout,
                    &vec![view_resource, sampler_resource],
                )
                .into(),
            )
            .unwrap();
        self
    }

    pub fn set_shader<'life>(&'life mut self, render_pass: &mut RenderPass<'life>) {
        self.shader
            .set_binding(3, self.model_info.get_bind_group())
            .unwrap();
        self.shader.apply(render_pass);
    }

    pub fn inner(&self) -> SmartShader {
        self.shader.clone()
    }
}

impl Shader3D for Smart3D {}
