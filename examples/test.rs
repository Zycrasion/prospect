use std::time::{SystemTime, Duration};

use prospect::{
    abstraction::{
        high_level_abstraction::HighLevelGraphicsContext,
        mesh::{Mesh, Meshable},
        prospect_window::ProspectWindow,
        shader::BasicShader,
        vertex::Vertex, graphics_context::GraphicsContext,
    },
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
    prospect_shape::ProspectShape, shaders::{textured_shader::TexturedShader, default_3d::Default3D}, prospect_camera::ProspectCamera, prospect_camera_controller::CameraController, prospect_light::ProspectPointLight, model::Model3D,
};
use prospect_obj::parse_obj;
use vecto_rs::{linear::{Vector, VectorTrait}, trig::to_degrees};
use wgpu::SurfaceError;
use winit::{event::{ElementState, VirtualKeyCode, MouseButton}, window::CursorGrabMode};

fn main() {
    let mut window = ProspectWindow::new("Test Window", 480, 480);

    let app = TestApp::new(&mut window);
    window.run_with_app(Box::new(app));
}

fn to_shape(str : &str) -> ProspectShape<Vec<Vertex>, Vec<u16>>
{

    let mut mesh = parse_obj(str);
    let verts = mesh.extract_vertices_and_uv_and_normals();
    let mut shape : ProspectShape<Vec<Vertex>, Vec<u16>> = ProspectShape { vertices: Vec::new(), indices: None };

    for vert in verts
    {
        shape.vertices.push(Vertex { position: [vert.0.x, vert.0.y, vert.0.z], uv: [vert.1.x, 1. - vert.1.y], normal : [vert.2.x, vert.2.y, vert.2.z] })
    }

    shape
}

pub struct TestApp {
    light_mesh: Mesh,
    light_model : Model3D,
    car_mesh : Mesh,
    mario_mesh : Mesh,
    car1: Model3D,
    mario : Model3D,
    draw_triangle: bool,
    frame : f32,
    camera: ProspectCamera,
    cam_controller : CameraController,
    last_frame : SystemTime,
    light : ProspectPointLight
}

impl TestApp {
    fn new(window: &mut ProspectWindow) -> Self {
        let camera = ProspectCamera::new(window.get_device());
        let mut light  = ProspectPointLight::new(window);
        light.position = Vector::new3(4., 4., 4.);
        light.colour   = Vector::new3(1., 1., 1.);

        let default_shader = Default3D::new(&window);
        let default_shader_key = window.add_shader(&default_shader, &camera, vec![light.get_layout()]);

        let car_texture = default_shader.register_texture("Car Texture", include_bytes!("../res/car01_Car_Pallete.png"), window);
        let light_texture = default_shader.register_texture("Light Texture", include_bytes!("../res/light.png"), window);
        let mario_texture = default_shader.register_texture("Mario Texture", include_bytes!("../res/mario.png"), window);

        let mut car_mesh = Mesh::from_shape(&to_shape(include_str!("../res/car01.obj")), window.get_device(), &default_shader_key);
        car_mesh.set_bind_group(1, &car_texture);
        car_mesh.set_bind_group(2, light.get_bind_index());
        let car1 = Model3D::new(&default_shader, window);

        let mut mario_mesh = Mesh::from_shape(&to_shape(include_str!("../res/mario.obj")), window.get_device(), &default_shader_key);
        mario_mesh.set_bind_group(1, &mario_texture);
        mario_mesh.set_bind_group(2, light.get_bind_index());
        let mario = Model3D::new(&default_shader, window);
        
        let mut light_mesh = Mesh::from_shape(&to_shape(include_str!("../res/light.obj")), window.get_device(), &default_shader_key);
        light_mesh.set_bind_group(1, &light_texture);
        light_mesh.set_bind_group(2, light.get_bind_index());

        let light_model = Model3D::new(&default_shader, window);

        Self {
            light_mesh,
            light_model,
            mario_mesh,
            car_mesh,
            car1,
            mario,
            draw_triangle: true,
            frame: 1.,
            camera,
            last_frame : SystemTime::now(),
            cam_controller : CameraController::new(),
            light
        }
    }
}

impl ProspectApp for TestApp {
    fn setup(&mut self, window: &mut ProspectWindow) {}

    fn draw(&mut self, window: &mut ProspectWindow) -> Result<(), SurfaceError> {
        /* update */
        let this_time = SystemTime::now();
        let delta = this_time.duration_since(self.last_frame).unwrap_or(Duration::from_secs_f32(1. / 60.)).as_secs_f32();
        self.last_frame = this_time;

        self.light.process_frame(window);
        self.cam_controller.process(delta, &mut self.camera, window);
        self.camera.process_frame(window.size.0 as f32, window.size.1 as f32, window.get_queue());

        self.car1.transform.rotation.y += to_degrees(delta);
        self.mario.transform.position.x = (self.frame / 2.).sin() * 5.;
        self.mario.transform.position.z = (self.frame / 2.).cos() * 5.;
        self.mario.transform.scale = 0.1;
        self.light_model.transform.position = self.light.position;

        let clear_colour = (
            0.5,
            0.0,
            0.5,
        );

        /* draw */
        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass =
            HighLevelGraphicsContext::start_render(clear_colour, &view, window.get_depth_buffer(), &mut command_encoder);
        
        self.light_model.draw(&mut render_pass, window, &self.camera, &self.light_mesh);
        self.car1.draw(&mut render_pass, window, &self.camera, &self.car_mesh);
        self.mario.draw(&mut render_pass, window, &self.camera, &self.mario_mesh);

        drop(render_pass);

        self.frame += 1. / 60.;
        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        Ok(())
    }

    fn process(&mut self, event: ProspectEvent, window: &mut ProspectWindow) -> ProcessResponse {
        match event {
            ProspectEvent::KeyboardInput(key, ElementState::Pressed) => {
                if key == Some(VirtualKeyCode::Tab) {
                    self.draw_triangle = !self.draw_triangle;
                }

                if key == Some(VirtualKeyCode::Q) {
                    window.lock_cursor(CursorGrabMode::None).unwrap();
                }
                
                if key.is_some()
                {
                    self.cam_controller.key_pressed(key.expect("Unexpected None for CameraController"));
                }

                if key == Some(VirtualKeyCode::Escape) {
                    ProcessResponse::CloseApp
                } else {
                    ProcessResponse::DontProcess
                }
            }
            ProspectEvent::KeyboardInput(key, ElementState::Released) => {
                if key.is_some()
                {
                    self.cam_controller.key_released(key.expect("Unexpected None for CameraController"));
                }

                ProcessResponse::DontProcess
            }
            ProspectEvent::CursorDelta(delta) =>
            {
                self.cam_controller.mouse_delta(delta);

                ProcessResponse::DontProcess
            }
            ProspectEvent::CursorMoveEvent(cursor_pos) => {
                self.cam_controller.mouse_move_event(cursor_pos, window);

                ProcessResponse::DontProcess
            }
            ProspectEvent::CursorClicked(state, button) =>
            {
                match button
                {
                    MouseButton::Right =>
                    {
                        self.cam_controller.mouse_click_event(state, window)
                    }
                    _ => {}
                }
                ProcessResponse::DontProcess
            }
            _ => {ProcessResponse::ProspectProcess}
        }
    }
}
