use prospect::{
    abstraction::{prospect_window::ProspectWindow, graphics_context::GraphicsContext, high_level_abstraction::HighLevelGraphicsContext},
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
};
use winit::{event::VirtualKeyCode, window};

fn main() {
    let window = ProspectWindow::new("Pong", 480, 480);

    window.run_with_app(Box::new(PongApp))
}

pub struct PongApp;

impl ProspectApp for PongApp {
    fn setup(&mut self) {}

    fn draw(&mut self, window : &ProspectWindow)
    {
        let clear_colour = (0.2, 0.2, 0.5);
        let (output, _view, command_encoder) = HighLevelGraphicsContext::start_render(window, clear_colour);
        
        HighLevelGraphicsContext::finish_render(window, command_encoder, output);
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
            }
        }
    }
}
