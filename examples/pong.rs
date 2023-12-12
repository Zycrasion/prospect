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

const PENTAGON: ProspectShape<&[Vertex], &[u16]> = ProspectShape {
    vertices: &[
        Vertex {
            position: [-0.0868241, 0.49240386, 0.0],
            uv: [-0.0868241, 0.49240386],
            normal: [0.; 3]
        }, // A
        Vertex {
            position: [-0.49513406, 0.06958647, 0.0],
            uv: [-0.49513406, 0.06958647],
            normal: [0.; 3]
        }, // B
        Vertex {
            position: [-0.21918549, -0.44939706, 0.0],
            uv: [-0.21918549, -0.44939706],
            normal: [0.; 3]
        }, // C
        Vertex {
            position: [0.35966998, -0.3473291, 0.0],
            uv: [0.35966998, -0.3473291],
            normal: [0.; 3]
        },// D
        Vertex {
            position: [0.44147372, 0.2347359, 0.0],
            uv: [0.44147372, 0.2347359],
            normal: [0.; 3]
        }, // E
    ],
    indices: Some(&[0u16, 1, 4, 1, 2, 4, 2, 3, 4]),
};

const TRIANGLE: ProspectShape<&[Vertex], &[u16]> = ProspectShape {
    vertices: &[
        Vertex {
            position: [0.0, 0.5, 0.],
            uv : [1., 0.],
            normal : [0.; 3]
        },
        Vertex {
            position: [0.5, -0.5, 0.],
            uv : [0., 1.],
            normal : [0.; 3]
        },
        Vertex {
            position: [-0.5, -0.5, 0.],
            uv : [1., 1.],
            normal : [0.; 3]
        },
    ],
    indices: None,
};

fn main() {
    let mut window = ProspectWindow::new("Pong", 480, 480);

    let app = PongApp::new(&mut window);
    window.run_with_app(Box::new(app))
}

pub struct PongApp {
    triangle_mesh: Mesh,
    light_model : Model3D,
    car_mesh : Mesh,
    car1: Model3D,
    car2 : Model3D,
    draw_triangle: bool,
    frame : f32,
    camera: ProspectCamera,
    cam_controller : CameraController,
    last_frame : SystemTime,
    light : ProspectPointLight
}


impl PongApp {
    fn new(window: &mut ProspectWindow) -> Self {
        let camera = ProspectCamera::new(window.get_device());
        let mut light  = ProspectPointLight::new(window);
        light.position = Vector::new3(4., 4., 4.);
        light.colour   = Vector::new3(1., 1., 1.);

        let default_shader = Default3D::new(&window);
        let default_shader_key = window.add_shader(&default_shader, &camera, vec![light.get_layout()]);
        let main_shader = BasicShader::new(&window);
        let bad_shader = window.add_shader(&main_shader, &camera, vec![]);

        let texture = default_shader.register_texture("Car Texture", include_bytes!("../res/car01_Car_Pallete.png"), window);

        let mut car_mesh = parse_obj(include_str!("../res/car01.obj"));
        let car_mesh_verts = car_mesh.extract_vertices_and_uv_and_normals();
        let mut shape : ProspectShape<Vec<Vertex>, Vec<u16>> = ProspectShape { vertices: Vec::new(), indices: None };

        for vert in car_mesh_verts
        {
            shape.vertices.push(Vertex { position: [vert.0.x, vert.0.y, vert.0.z], uv: [vert.1.x, 1. - vert.1.y], normal : [vert.2.x, vert.2.y, vert.2.z] })
        }

        let mut car_mesh = Mesh::from_shape(&shape, window.get_device(), &default_shader_key);
        car_mesh.set_bind_group(1, &texture);
        car_mesh.set_bind_group(2, light.get_bind_index());
        let car1 = Model3D::new(&default_shader, window);
        let car2 = Model3D::new(&default_shader, window);
        
        let triangle_mesh = Mesh::from_shape(&TRIANGLE, window.get_device(), &bad_shader);

        let light_model = Model3D::new(&default_shader, window);

        Self {
            triangle_mesh,
            light_model,
            car_mesh,
            car1 : car1,
            car2 : car2,
            draw_triangle: true,
            frame: 1.,
            camera,
            last_frame : SystemTime::now(),
            cam_controller : CameraController::new(),
            light
        }
    }
}

impl ProspectApp for PongApp {
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
        self.car2.transform.position.x = self.frame.sin() * 5.;
        self.car2.transform.position.z = self.frame.cos() * 5.;
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
        

        self.triangle_mesh.draw(&mut render_pass, window.get_shader_manager(), &self.camera);

        /// TODO: Get better Light Mesh
        self.light_model.draw(&mut render_pass, window, &self.camera, &self.car_mesh);
        self.car1.draw(&mut render_pass, window, &self.camera, &self.car_mesh);
        self.car2.draw(&mut render_pass, window, &self.camera, &self.car_mesh);

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
