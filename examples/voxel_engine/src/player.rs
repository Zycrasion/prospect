use prospect::{prospect_camera::ProspectCamera, prospect_camera_controller::CameraController, abstraction::prospect_window::ProspectWindow, prospect_app::ProspectEvent, winit::window, linear::vector3};

pub struct Player
{
    camera : ProspectCamera,
    controller : CameraController 
}

impl Player
{
    pub fn new(window : &mut ProspectWindow) -> Self
    {
        let mut controller = CameraController::new();
        let mut camera = ProspectCamera::new(window.get_device());

        controller.sprint_multiplier = 50.;


        Self
        {
            camera: camera,
            controller
        }
    }

    pub fn update(&mut self, window : &mut ProspectWindow)
    {
        self.controller.process(window.get_deltaf32(), &mut self.camera, window);
        self.camera.process_frame(
            window.size.0 as f32,
            window.size.1 as f32,
            window.get_queue(),
        );
    }

    pub fn process(&mut self, event : ProspectEvent, window : &mut ProspectWindow) {
        self.controller.input_event(event, window);
    }

    pub fn get_camera(&self) -> &ProspectCamera
    {
        &self.camera
    }
}