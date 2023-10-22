use prospect::{
    abstraction::prospect_window::ProspectWindow,
    prospect_app::{ProcessResponse, ProspectApp, ProspectEvent},
};
use winit::event::VirtualKeyCode;

fn main() {
    let window = ProspectWindow::new("Pong", 480, 480);

    window.run_with_app(Box::new(PongApp))
}

pub struct PongApp;

impl ProspectApp for PongApp {
    fn setup(&mut self) {}

    fn draw(&mut self) {}

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
