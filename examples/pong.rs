use prospect::{abstraction::prospect_window::ProspectWindow, prospect_app::ProspectApp};

fn main()
{
    let window = ProspectWindow::new("Pong", 480, 480);

    window.run_with_app(Box::new(PongApp))
}

pub struct PongApp;

impl ProspectApp for PongApp
{
    fn setup(&mut self) {
      
    }

    fn draw(&mut self) {
        
    }

    fn process(&mut self, event : prospect::prospect_app::PropsectEvent) {
        
    }
}