use prospect::{
    abstraction::{prospect_window::ProspectWindow, graphics_context::GraphicsContext, high_level_abstraction::HighLevelGraphicsContext, mesh::{Mesh, Meshable}, vertex::Vertex, shader::BasicShader},
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent}, prospect_shape::ProspectShape, render_pipeline_index::RenderPipelineIndex,
};
use wgpu::SurfaceError;
use winit::{event::VirtualKeyCode, window};

const TRIANGLE : ProspectShape<&[Vertex], &[u16]> = ProspectShape
{
    vertices: &[
        Vertex { position : [  0.0,  0.5, 0.], colour : [1., 0., 0.] },
        Vertex { position : [  0.5, -0.5, 0.], colour : [0., 1., 0.] },
        Vertex { position : [ -0.5, -0.5, 0.], colour : [0., 0., 1.] },
    ],
    indices: None,
};

fn main() {
    let window = ProspectWindow::new("Hello World!", 480, 480);

    let a = Box::new(HelloWorld::new(&window));
    window.run_with_app(a);
}

pub struct HelloWorld
{
    clear_col : (f64, f64, f64),
    mesh : Mesh,
    shader_manager : RenderPipelineIndex
}

impl HelloWorld
{
    pub fn new(window : &ProspectWindow) -> Self
    {
        let mut shader_manager = RenderPipelineIndex::new();
        let main_shader = shader_manager.add_shader(&BasicShader::new(&window), &window).expect("Unable to add BasicShader");

        let mesh = Mesh::from_shape(&TRIANGLE, window.get_device(), &main_shader);

        Self
        {
            mesh,
            clear_col: (0., 0., 0.),
            shader_manager
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

        self.mesh.draw(&mut render_pass, &self.shader_manager);

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
