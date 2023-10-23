use prospect::{
    abstraction::{prospect_window::ProspectWindow, graphics_context::GraphicsContext, high_level_abstraction::HighLevelGraphicsContext},
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
};
use wgpu::SurfaceError;
use winit::{event::VirtualKeyCode, window};

fn main() {
    let window = ProspectWindow::new("Hello World!", 480, 480);

    window.run_with_app(Box::new(HelloWorld::default()))
}

#[derive(Default)]
pub struct HelloWorld
{
    clear_col : (f64, f64, f64)
}

impl ProspectApp for HelloWorld {
    fn setup(&mut self) {}

    fn draw(&mut self, window : &ProspectWindow) -> Result<(), SurfaceError>
    {
        let clear_colour = (self.clear_col.0 / window.size.0 as f64, self.clear_col.1 / window.size.1 as f64, 0.5);
        let (output, view, mut command_encoder) = HighLevelGraphicsContext::init_view(window);
        let mut render_pass = HighLevelGraphicsContext::start_render(clear_colour, &view, &mut command_encoder);
        render_pass.set_pipeline(window.get_render_pipeline());
        render_pass.draw(0..3, 0..1);
        
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
