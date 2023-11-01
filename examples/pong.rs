use prospect::{
    abstraction::{prospect_window::ProspectWindow, graphics_context::GraphicsContext, high_level_abstraction::HighLevelGraphicsContext, vertex::Vertex},
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
};
use wgpu::{SurfaceError, BufferUsages};
use wgpu::util::DeviceExt;
use winit::{event::VirtualKeyCode, window};

const VERTICES : &[Vertex] = &[
    Vertex { position : [0.0, 0.5, 0.0], colour : [1.0, 0.0, 0.0] },
    Vertex { position : [-0.5, -0.5, 0.0], colour : [0.0, 1.0, 0.0] },
    Vertex { position : [0.5, -0.5, 0.0], colour : [0.0, 0.0, 1.0] }
];

fn main() {
    let window = ProspectWindow::new("Pong", 480, 480);

    let app = PongApp::new(&window);
    window.run_with_app(Box::new(app))
}

pub struct PongApp
{
    clear_col : (f64, f64, f64),
    vertex_buffer : wgpu::Buffer,
    num_vertices : u32
}

impl PongApp
{
    fn new(window : &ProspectWindow) -> Self
    {
        let vertex_buffer = GraphicsContext::create_buffer(window.get_device(), "Vertex Buffer", VERTICES, BufferUsages::VERTEX);
        Self
        {
            clear_col : (0., 0. ,0.),
            vertex_buffer,
            num_vertices : VERTICES.len() as u32
        }
    }
}

impl ProspectApp for PongApp {
    fn setup(&mut self) {}

    fn draw(&mut self, window : &ProspectWindow) -> Result<(), SurfaceError>
    {
        let clear_colour = (self.clear_col.0 / window.size.0 as f64, self.clear_col.1 / window.size.1 as f64, 0.5);
        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass = HighLevelGraphicsContext::start_render(clear_colour, &view, &mut command_encoder);
        render_pass.set_pipeline(window.get_render_pipeline());
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);

        drop(render_pass);

        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
        Ok(())
    }

    fn process(&mut self, event: ProspectEvent) -> ProcessResponse {
        match event {
            ProspectEvent::KeyboardInput(key) => {
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
