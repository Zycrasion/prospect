use prospect::{
    abstraction::{prospect_window::ProspectWindow, graphics_context::GraphicsContext, high_level_abstraction::HighLevelGraphicsContext, vertex::Vertex, mesh::{Mesh, Meshable, MeshIndexed}},
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
};
use wgpu::{SurfaceError, BufferUsages};
use wgpu::util::DeviceExt;
use winit::{event::VirtualKeyCode, window};

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], colour: [0.5, 0.5, 0.] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], colour: [0.5, 0.0, 0.] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], colour: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], colour: [0., 0.5, 0.] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], colour: [0., 0.0, 0.5] }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];


fn main() {
    let window = ProspectWindow::new("Pong", 480, 480);

    let app = PongApp::new(&window);
    window.run_with_app(Box::new(app))
}

pub struct PongApp
{
    clear_col : (f64, f64, f64),
    triangle_mesh : Box<dyn Meshable>
}

impl PongApp
{
    fn new(window : &ProspectWindow) -> Self
    {
        let triangle_mesh = Box::new(MeshIndexed::new(VERTICES, INDICES, window.get_device()));

        Self
        {
            clear_col : (0., 0. ,0.),
            triangle_mesh
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
        self.triangle_mesh.draw(&mut render_pass);

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
