use prospect::{
    abstraction::{
        high_level_abstraction::HighLevelGraphicsContext,
        mesh::{Mesh, Meshable},
        prospect_window::ProspectWindow,
        shader::{BasicShader, self},
        vertex::{Vertex}, graphics_context::GraphicsContext,
    },
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
    prospect_shape::ProspectShape, prospect_shader_manager::ProspectShaderManager, shaders::{textured_shader::{TexturedShader, TexturedShaderTexture}},
};
use wgpu::SurfaceError;
use winit::event::{ElementState, VirtualKeyCode};

const PENTAGON: ProspectShape<&[Vertex], &[u16]> = ProspectShape {
    vertices: &[
        Vertex {
            position: [-0.0868241, 0.49240386, 0.0],
            uv: [-0.0868241, 0.49240386]
        }, // A
        Vertex {
            position: [-0.49513406, 0.06958647, 0.0],
            uv: [-0.49513406, 0.06958647]
        }, // B
        Vertex {
            position: [-0.21918549, -0.44939706, 0.0],
            uv: [-0.21918549, -0.44939706]
        }, // C
        Vertex {
            position: [0.35966998, -0.3473291, 0.0],
            uv: [0.35966998, -0.3473291]
        },// D
        Vertex {
            position: [0.44147372, 0.2347359, 0.0],
            uv: [0.44147372, 0.2347359]
        }, // E
    ],
    indices: Some(&[0u16, 1, 4, 1, 2, 4, 2, 3, 4]),
};

const TRIANGLE: ProspectShape<&[Vertex], &[u16]> = ProspectShape {
    vertices: &[
        Vertex {
            position: [0.0, 0.5, 0.],
            uv : [0., 0.]
        },
        Vertex {
            position: [0.5, -0.5, 0.],
            uv : [0., 0.]
        },
        Vertex {
            position: [-0.5, -0.5, 0.],
            uv : [0., 0.]
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
    clear_col: (f64, f64, f64),
    triangle_mesh: Mesh,
    pentagon_mesh: Mesh,
    draw_triangle: bool
}


impl PongApp {
    fn new(window: &mut ProspectWindow) -> Self {
        let image_shader = TexturedShader::new(&window);
        let image_shader_key = window.add_shader(&image_shader).expect("Unable to register TexturedShader");
        let main_shader = BasicShader::new(&window);
        let main_shader = window.add_shader(&main_shader).expect("Unable to register main_shader");

        let texture = GraphicsContext::create_texture("car01_Car_Pallete.png", include_bytes!("../res/car01_Car_Pallete.png"), &window.get_device(), &window.get_queue());
        let texture = GraphicsContext::create_texture_view(&texture);
        let texture = image_shader.create_texture(window, &texture, "car01_Car_Pallete.png");
        let texture = window.add_bind_group("Car Texture", texture.1).expect("Unable to register Car Texture");
        
        let mut pentagon_mesh = Mesh::from_shape(&PENTAGON, window.get_device(), &image_shader_key);
        pentagon_mesh.set_bind_group(0, &texture);
        
        let triangle_mesh = Mesh::from_shape(&TRIANGLE, window.get_device(), &main_shader);

        Self {
            clear_col: (0., 0., 0.),
            triangle_mesh,
            pentagon_mesh,
            draw_triangle: true
        }
    }
}

impl ProspectApp for PongApp {
    fn setup(&mut self) {}

    fn draw(&mut self, window: &ProspectWindow) -> Result<(), SurfaceError> {
        let clear_colour = (
            self.clear_col.0 / window.size.0 as f64,
            self.clear_col.1 / window.size.1 as f64,
            0.5,
        );

        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass =
            HighLevelGraphicsContext::start_render(clear_colour, &view, &mut command_encoder);

        if !self.draw_triangle {
            self.triangle_mesh.draw(&mut render_pass, window.get_shader_manager());
        } else {
            self.pentagon_mesh.draw(&mut render_pass, window.get_shader_manager());
        }

        drop(render_pass);

        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        Ok(())
    }

    fn process(&mut self, event: ProspectEvent) -> ProcessResponse {
        match event {
            ProspectEvent::KeyboardInput(key, ElementState::Pressed) => {
                if key == Some(VirtualKeyCode::Space) {
                    self.draw_triangle = !self.draw_triangle;
                }

                if key == Some(VirtualKeyCode::Escape) {
                    ProcessResponse::CloseApp
                } else {
                    ProcessResponse::DontProcess
                }
            }
            ProspectEvent::KeyboardInput(_key, ElementState::Released) => {
                ProcessResponse::DontProcess
            }
            ProspectEvent::CursorMoveEvent(cursor_pos) => {
                self.clear_col.0 = cursor_pos.x as f64;
                self.clear_col.1 = cursor_pos.y as f64;
                ProcessResponse::DontProcess
            }
        }
    }
}
