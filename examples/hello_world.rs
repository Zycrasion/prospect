use prospect::{
    abstraction::{prospect_window::ProspectWindow, graphics_context::GraphicsContext, high_level_abstraction::HighLevelGraphicsContext, mesh::{Mesh, Meshable}, vertex::VertexPOSCOL, shader::BasicShader},
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent}, prospect_shape::ProspectShape, prospect_shader_manager::ProspectShaderManager,
};
use wgpu::SurfaceError;
use winit::{event::VirtualKeyCode, window};

const TRIANGLE : ProspectShape<&[VertexPOSCOL], &[u16]> = ProspectShape
{
    vertices: &[
        VertexPOSCOL { position : [  0.0,  0.5, 0.], colour : [1., 0., 0.] },
        VertexPOSCOL { position : [  0.5, -0.5, 0.], colour : [0., 1., 0.] },
        VertexPOSCOL { position : [ -0.5, -0.5, 0.], colour : [0., 0., 1.] },
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
}

impl HelloWorld
{
    pub fn new(window : &mut ProspectWindow) -> Self
    {
        let basic_shader = BasicShader::new(window);
        let basic_shader = window.add_shader(&basic_shader).expect("Unable to register BasicShader");

        let mesh = Mesh::from_shape(&TRIANGLE, window.get_device(), &basic_shader);

        Self
        {
            mesh,
            clear_col: (0., 0., 0.),
        }
    }
}

impl ProspectApp for HelloWorld {
    fn setup(&mut self) {}

    fn draw(&mut self, window : &ProspectWindow) -> Result<(), SurfaceError>
    {
        let clear_colour = (self.clear_col.0 / window.size.0 as f64, self.clear_col.1 / window.size.1 as f64, 0.5);
        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass = HighLevelGraphicsContext::start_render(clear_colour, &view, &mut command_encoder);

        self.mesh.draw(&mut render_pass, window.get_shader_manager());

        drop(render_pass);

        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        Ok(())
    }

    fn process(&mut self, event: ProspectEvent) -> ProcessResponse {
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
            }
        }
    }
}
