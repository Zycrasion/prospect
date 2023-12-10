use prospect::{
    abstraction::{prospect_window::ProspectWindow, high_level_abstraction::HighLevelGraphicsContext, mesh::{Mesh, Meshable}, shader::BasicShader, vertex::Vertex},
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent}, prospect_shape::ProspectShape, prospect_camera::ProspectCamera,
};
use vecto_rs::linear::{Vector, VectorTrait};
use wgpu::SurfaceError;
use winit::event::VirtualKeyCode;

const TRIANGLE : ProspectShape<&[Vertex], &[u16]> = ProspectShape
{
    vertices: &[
        Vertex { position : [  0.0,  0.5, 0.], uv : [0.5, 0.0] },
        Vertex { position : [  0.5, -0.5, 0.], uv : [1.0, 1.0] },
        Vertex { position : [ -0.5, -0.5, 0.], uv : [0.0, 1.0] },
    ],
    indices: None,
};

fn main() {
    let mut window = ProspectWindow::new("Hello World!", 480, 480);

    let a = Box::new(HelloWorld::new(&mut window));
    window.run_with_app(a);
}

pub struct HelloWorld
{
    clear_col : (f64, f64, f64),
    mesh : Mesh,
    camera : ProspectCamera
}

impl HelloWorld
{
    pub fn new(window : &mut ProspectWindow) -> Self
    {
        let basic_shader = BasicShader::new(window);
        let mut camera = ProspectCamera::new(window.get_device());
        camera.eye = Vector::new3(0., 0., 1.);
        let basic_shader = window.add_shader(&basic_shader, &camera);

        let mesh = Mesh::from_shape(&TRIANGLE, window.get_device(), &basic_shader);

        Self
        {
            mesh,
            clear_col: (0., 0., 0.),
            camera
        }
    }
}

impl ProspectApp for HelloWorld {
    fn setup(&mut self, window : &mut ProspectWindow) {}

    fn draw(&mut self, window : &mut ProspectWindow) -> Result<(), SurfaceError>
    {
        let clear_colour = (self.clear_col.0 / window.size.0 as f64, self.clear_col.1 / window.size.1 as f64, 0.5);
        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass = HighLevelGraphicsContext::start_render(clear_colour, &view, window.get_depth_buffer(), &mut command_encoder);

        self.mesh.draw(&mut render_pass, window.get_shader_manager(), &self.camera);

        drop(render_pass);

        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        Ok(())
    }

    fn process(&mut self, event: ProspectEvent, window : &mut ProspectWindow) -> ProcessResponse {
        match event {
            ProspectEvent::KeyboardInput(key, _) => {
                if key == Some(VirtualKeyCode::Escape) {
                    ProcessResponse::CloseApp
                } else
                {
                    ProcessResponse::DontProcess
                }
            },
            ProspectEvent::CursorMoveEvent(cursor_pos) =>
            {
                self.clear_col.0 = cursor_pos.x as f64;
                self.clear_col.1 = cursor_pos.y as f64;
                ProcessResponse::DontProcess
            },
            _ => {ProcessResponse::DontProcess}
        }
    }
}
