use std::path::Path;
use std::time::{Duration, SystemTime};


use noise::{Perlin, NoiseFn};
use prospect::parse_obj;
use prospect::prospect_shader_manager::{ProspectBindGroupIndex, ProspectShaderIndex};
use prospect::trig::to_radians;
use prospect::utils::prospect_fs::{path_with_respect_to_cwd_str, path_with_respect_to_cwd};
use prospect::wgpu::{SurfaceError, Texture, PrimitiveTopology};
use prospect::winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::CursorGrabMode,
};
use prospect::{
    abstraction::{
        graphics_context::GraphicsContext,
        high_level_abstraction::HighLevelGraphicsContext,
        mesh::{Mesh, Meshable},
        prospect_window::ProspectWindow,
        shader::BasicShader,
        vertex::Vertex,
    },
    model::Model3D,
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
    prospect_camera::ProspectCamera,
    prospect_camera_controller::CameraController,
    prospect_light::ProspectPointLight,
    prospect_shape::ProspectShape,
    shaders::{default_3d::Default3D, textured_shader::TexturedShader},
    utils::prospect_fs::{
        read_file_panic, read_file_with_respect_to_cwd, read_file_with_respect_to_cwd_bytes,
    },
};
use prospect::{
    linear::{Vector, VectorTrait},
    trig::to_degrees,
};

fn generate_height(x : f32, y : f32, perlin : Perlin, z : f32) -> f32
{
    perlin.get([x as f64 / 3., y as f64 / 3., z as f64]) as f32
}

fn generate_vertex(x : f32, z : f32, perlin : Perlin) -> Vertex
{
    let b = Vector::new3(x, generate_height(x, z, perlin, 0.), z);
    let a = Vector::new3(x + 0.1, generate_height(x + 0.1, z, perlin, 0.), z);
    let c = Vector::new3(x, generate_height(x, z + 0.1, perlin, 0.), z + 0.1);
    
    let ab = b - a;
    let bc = c - b;

    let normal = Vector::cross(&ab, &bc).normalized();

    let y = generate_height(x, z, perlin, 0.);
    let uv_x = 1. - (y / 2. + 0.5);
    let uv_y_vary = perlin.get([x as f64 / 10., z as f64 / 10., 100.]) as f32;
    let uv_y = 5. / 16. + uv_y_vary / 16.;

    Vertex
    {
        position : [x, y, z],
        uv : [uv_x, uv_y]  ,
        normal : [normal.x, normal.y, normal.z]
    }
}


fn main() {
    let mut window = ProspectWindow::new("Test Window", 480, 480);
    let app = ObjPreviewer::new(&mut window);
    window.run_with_app(Box::new(app));
}

pub struct ObjPreviewer {
    elapsed : f32,
    main_model: Model3D,
    main_mesh: Mesh,
    camera: ProspectCamera,
    cam_controller: CameraController,
    last_frame: SystemTime,
    light: ProspectPointLight,
    light_mesh : Mesh,
    light_model : Model3D
}

impl ObjPreviewer {
    fn new(window: &mut ProspectWindow) -> Self {
        let camera = ProspectCamera::new(window.get_device());
        let mut light = ProspectPointLight::new(window);
        light.position = Vector::new3(4., 4., 4.);
        light.colour = Vector::new3(1., 1., 1.);

        let default_shader = Default3D::new(&window);
        let default_shader_key =
            window.add_shader(&default_shader, &camera, vec![light.get_layout()]);

        let light_texture = default_shader.register_texture("Light Texture", include_bytes!("../res/light.png"), window);
        let mut light_mesh = Mesh::from_shape(&to_shape(include_str!("../res/light.obj")), window.get_device(), &default_shader_key);
        light_mesh.set_bind_group(1, &light_texture);
        light_mesh.set_bind_group(2, light.get_bind_index());

        /* Terrain */

        let terrain_shader = Default3D::new_with_custom_topology(&window, PrimitiveTopology::TriangleList);
        let terrain_shader_key =
            window.add_shader(&terrain_shader, &camera, vec![light.get_layout()]);

        let pallete = terrain_shader.register_texture(
            "texture",
            include_bytes!("../res/pallete01.png"),
            window,
        );

        let mut shape : ProspectShape<Vec<Vertex>, Vec<u32>> = ProspectShape { vertices: vec![], indices: Some(vec![]) };

        let mut indices = vec![];
        let size = 2u32.pow(11) + 2u32.pow(8);
        let perlin = Perlin::new(0);

        let mut vert_count = 0u32;
        let mut face_count = 0u32;

        for z in 0..size
        {
            for x in 0..size
            {
                shape.vertices.push(generate_vertex((x as f32 - size as f32 / 2.) / 4., (z as f32 - size as f32 / 2.) as f32 / 4., perlin));
                vert_count += 1;
                if x + 1 < size && z + 1 < size
                {
                    indices.push(z * size + x + 1);
                    indices.push((z + 1) * size + x);
                    indices.push(z * size + x);
                    face_count += 1;

                    indices.push((z + 1) * size + x + 1);
                    indices.push((z + 1) * size + x);
                    indices.push(z * size + x + 1);
                    face_count += 1;
                }
            }
        }

        println!("verts : {vert_count} faces : {face_count} indices : {}", indices.len());
        shape.indices = Some(indices);

        let mut main_mesh = Mesh::from_shape(
            &shape,
            window.get_device(),
            &terrain_shader_key,
        );
        main_mesh.set_bind_group(1, &pallete);
        main_mesh.set_bind_group(2, light.get_bind_index());
        let main_model = Model3D::new(&terrain_shader, window);

        let light_model = Model3D::new(&terrain_shader, window);

        // Dispatch watcher
        Self {
            elapsed : 0.,
            main_mesh,
            main_model,
            light_mesh,
            light_model,
            camera,
            last_frame: SystemTime::now(),
            cam_controller: CameraController::new(),
            light,
        }
    }
}

impl ProspectApp for ObjPreviewer {
    fn setup(&mut self, window: &mut ProspectWindow) {}

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

        self.light_model.draw(&mut render_pass, window, &self.camera, &self.light_mesh);
        self.main_model
            .draw(&mut render_pass, window, &self.camera, &self.main_mesh);

        drop(render_pass);

        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        self.elapsed += delta;
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

fn to_shape(str : &str) -> ProspectShape<Vec<Vertex>, Vec<u32>>
{

    let mut mesh = parse_obj(str);
    let verts = mesh.extract_vertices_and_uv_and_normals();
    let mut shape : ProspectShape<Vec<Vertex>, Vec<u32>> = ProspectShape { vertices: Vec::new(), indices: None };

    for vert in verts
    {
        shape.vertices.push(Vertex { position: [vert.0.x, vert.0.y, vert.0.z], uv: [vert.1.x, 1. - vert.1.y], normal : [vert.2.x, vert.2.y, vert.2.z] })
    }

    shape
}