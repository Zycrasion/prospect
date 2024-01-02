use std::time::{Duration, SystemTime};
use prospect::abstraction::shader::ProspectShader;
use prospect::parse_obj;
use prospect::prospect_shader_manager::{ProspectBindGroupIndex, ProspectShaderIndex};
use prospect::prospect_texture::ProspectTexture;
use prospect::trig::to_radians;

use prospect::wgpu::*;
use prospect::winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::CursorGrabMode,
};
use prospect::{
    abstraction::{
        high_level_abstraction::HighLevelGraphicsContext,
        mesh::Mesh,
        prospect_window::ProspectWindow,
        vertex::Vertex,
    },
    model::Model3D,
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
    prospect_camera::ProspectCamera,
    prospect_camera_controller::CameraController,
    prospect_light::ProspectPointLight,
    prospect_shape::ProspectShape,
    shaders::default_3d::Default3D,
};
use prospect::
    linear::{Vector, VectorTrait}
;
use simple_terrain_gen::chunk::Chunk;

fn main() {
    let mut window = ProspectWindow::new("Test Window", 480, 480);
    let app = SimpleTerrainGen::new(&mut window);
    window.run_with_app(Box::new(app));
}

pub struct SimpleTerrainGen {
    elapsed: f32,
    camera: ProspectCamera,
    virtual_camera: ProspectCamera,
    cam_controller: CameraController,
    last_frame: SystemTime,
    light: ProspectPointLight,
    light_mesh: Mesh,
    light_model: Model3D,
    chunks: Vec<(Mesh, Model3D)>,
    update_virtual_camera: bool,
}

impl SimpleTerrainGen {
    fn new(window: &mut ProspectWindow) -> Self {
        let camera = ProspectCamera::new(window.get_device());
        let mut light = ProspectPointLight::new(window);
        light.position = Vector::new3(4., 4., 4.);
        light.colour = Vector::new3(1., 1., 1.);

        let default_shader = Default3D::new(&window);
        let default_shader_key =
            window.add_shader(&default_shader, &camera, vec![light.get_layout()]);

        let light_texture = default_shader.register_texture(
            "Light Texture",
            include_bytes!("../res/light.png"),
            window,
        );
        let mut light_mesh = Mesh::from_shape(
            &to_shape(include_str!("../res/light.obj")),
            window.get_device(),
            &default_shader_key,
        );
        light_mesh.set_bind_group(1, &light_texture);
        light_mesh.set_bind_group(2, light.get_bind_index());

        /* Terrain */

        let terrain_shader =
            Default3D::new_with_custom_topology(&window, PrimitiveTopology::TriangleList);
        let terrain_shader_key =
            window.add_shader(&terrain_shader, &camera, vec![light.get_layout()]);

        let pallete = ProspectTexture::from_bytes(
            "Pallete Texture",
            include_bytes!("../res/pallete.png"),
            window,
        );
        let pallete_terrain_shader = terrain_shader.bind_prospect_texture(&pallete, window);
        let light_model = Model3D::new(&terrain_shader, window);

        let mut chunks = vec![];

        for x in -5..=5 {
            for z in -5..=5 {
                chunks.push(Self::new_chunk(
                    x as f32 * 50. / 4.,
                    z as f32 * 50. / 4.,
                    window,
                    terrain_shader_key.clone(),
                    pallete_terrain_shader.clone(),
                    &light,
                    &terrain_shader,
                ));
            }
        }

        Self {
            elapsed: 0.,
            light_mesh,
            light_model,
            virtual_camera: ProspectCamera::new_from(window.get_device(), &camera),
            camera,
            last_frame: SystemTime::now(),
            cam_controller: CameraController::new(),
            light,
            chunks,
            update_virtual_camera: true,
        }
    }

    fn new_chunk(
        x: f32,
        z: f32,
        window: &mut ProspectWindow,
        terrain_shader_key: ProspectShaderIndex,
        pallete_terrain_shader: ProspectBindGroupIndex,
        light: &ProspectPointLight,
        shader: &impl ProspectShader,
    ) -> (Mesh, Model3D) {
        let shape: ProspectShape<Vec<Vertex>, Vec<u32>> = Chunk::generate(x, z);

        let mut main_mesh = Mesh::from_shape(&shape, window.get_device(), &terrain_shader_key);
        main_mesh.set_bind_group(1, &pallete_terrain_shader);
        main_mesh.set_bind_group(2, light.get_bind_index());

        let mut model = Model3D::new(shader, window);
        model.transform.position = Vector::new3(x, 0., z);

        (main_mesh, model)
    }
}

impl ProspectApp for SimpleTerrainGen {
    fn setup(&mut self, _window: &mut ProspectWindow) {}

    fn draw(&mut self, window: &mut ProspectWindow) -> Result<(), SurfaceError> {
        /* update */
        let this_time = SystemTime::now();
        let delta = this_time
            .duration_since(self.last_frame)
            .unwrap_or(Duration::from_secs_f32(1. / 60.))
            .as_secs_f32();
        self.last_frame = this_time;

        self.light.process_frame(window);
        self.cam_controller.process(delta, &mut self.camera, window);
        self.camera.process_frame(
            window.size.0 as f32,
            window.size.1 as f32,
            window.get_queue(),
        );

        if self.update_virtual_camera {
            self.virtual_camera.eye = self.camera.eye;
            self.virtual_camera.rotation = self.camera.rotation;
        }

        let clear_colour = (0.5, 0.0, 0.5);

        self.light.position.x = to_radians(self.elapsed as f32 * 10.).sin() * 10.;
        self.light.position.z = to_radians(self.elapsed as f32 * 10.).cos() * 10.;

        self.light_model.transform.position = self.light.position;

        /* draw */
        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass = HighLevelGraphicsContext::start_render(
            clear_colour,
            &view,
            window.get_depth_buffer(),
            &mut command_encoder,
        );

        self.light_model
            .draw(&mut render_pass, window, &self.camera, &self.light_mesh);

        let mut marked_chunks = vec![];

        {
            let mut index = 0;
            for chunk in &self.chunks {
                if chunk.1.transform.position.dist(&self.virtual_camera.eye)
                    <= self.virtual_camera.zfar
                {
                    chunk
                        .1
                        .draw(&mut render_pass, &window, &self.camera, &chunk.0);
                    index += 1;
                } else {
                    marked_chunks.push(index);
                }
            }
        }

        drop(render_pass);

        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        self.elapsed += delta;
        
        for chunk in marked_chunks {
            self.chunks.remove(chunk);
        }
        
        Ok(())
    }

    fn process(&mut self, event: ProspectEvent, window: &mut ProspectWindow) -> ProcessResponse {
        match event {
            ProspectEvent::KeyboardInput(key, ElementState::Pressed) => {
                if key == Some(VirtualKeyCode::Q) {
                    window.lock_cursor(CursorGrabMode::None).unwrap();
                }

                if key.is_some() {
                    self.cam_controller
                        .key_pressed(key.expect("Unexpected None for CameraController"));
                }

                if key == Some(VirtualKeyCode::Z) {
                    self.update_virtual_camera = !self.update_virtual_camera;
                    println!(
                        "{} Updating Virtual Camera!",
                        if self.update_virtual_camera {
                            "Started"
                        } else {
                            "Stopped"
                        }
                    );
                }

                if key == Some(VirtualKeyCode::Escape) {
                    ProcessResponse::CloseApp
                } else {
                    ProcessResponse::DontProcess
                }
            }
            ProspectEvent::KeyboardInput(key, ElementState::Released) => {
                if key.is_some() {
                    self.cam_controller
                        .key_released(key.expect("Unexpected None for CameraController"));
                }

                ProcessResponse::DontProcess
            }
            ProspectEvent::CursorDelta(delta) => {
                self.cam_controller.mouse_delta(delta);

                ProcessResponse::DontProcess
            }
            ProspectEvent::CursorMoveEvent(cursor_pos) => {
                self.cam_controller.mouse_move_event(cursor_pos, window);

                ProcessResponse::DontProcess
            }
            ProspectEvent::CursorClicked(state, button) => {
                match button {
                    MouseButton::Right => self.cam_controller.mouse_click_event(state, window),
                    _ => {}
                }
                ProcessResponse::DontProcess
            }
            _ => ProcessResponse::ProspectProcess,
        }
    }
}

fn to_shape(str: &str) -> ProspectShape<Vec<Vertex>, Vec<u32>> {
    let mut mesh = parse_obj(str);
    let verts = mesh.extract_vertices_and_uv_and_normals();
    let mut shape: ProspectShape<Vec<Vertex>, Vec<u32>> = ProspectShape {
        vertices: Vec::new(),
        indices: None,
    };

    for vert in verts {
        shape.vertices.push(Vertex {
            position: [vert.0.x, vert.0.y, vert.0.z],
            uv: [vert.1.x, 1. - vert.1.y],
            normal: [vert.2.x, vert.2.y, vert.2.z],
        })
    }

    shape
}
