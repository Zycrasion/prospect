use prospect::{
    abstraction::{prospect_window::ProspectWindow, graphics_context::GraphicsContext},
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
        let (output, view) = GraphicsContext::create_view(window.get_surface());
        let mut command_encoder = GraphicsContext::create_command_encoder(window.get_device(), "Draw Loop Commands");
        let render_pass = GraphicsContext::begin_render_pass_barebones((0.1, 0.1, 0.3, 1.0), "Render Pass", &view, &mut command_encoder);
        drop(render_pass);

        window.get_queue().submit(std::iter::once(command_encoder.finish()));
        output.present();
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
