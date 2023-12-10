use std::collections::{HashMap, HashSet};

use winit::event::VirtualKeyCode;

use crate::prospect_camera::ProspectCamera;

pub struct CameraController
{
    keys_down : HashSet<VirtualKeyCode>,
    pub units_per_second : f32
}

impl CameraController 
{
    pub fn new() -> Self
    {
        Self
        {
            keys_down : HashSet::new(),
            units_per_second : 1.
        }
    }

    pub fn process(&mut self, delta : f32, camera : &mut ProspectCamera)
    {
        if self.keys_down.contains(&VirtualKeyCode::W)
        {
            camera.eye.z -= self.units_per_second * delta;
        }

        if self.keys_down.contains(&VirtualKeyCode::S)
        {
            camera.eye.z += self.units_per_second * delta;
        }

        if self.keys_down.contains(&VirtualKeyCode::A)
        {
            camera.eye.x += self.units_per_second * delta;
        }
        
        if self.keys_down.contains(&VirtualKeyCode::D)
        {
            camera.eye.x -= self.units_per_second * delta;
        }
    }

    pub fn key_pressed(&mut self, key : VirtualKeyCode)
    {
        self.keys_down.insert(key);
    }

    pub fn key_released(&mut self, key : VirtualKeyCode)
    {
        self.keys_down.remove(&key);
    }
}