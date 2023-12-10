use std::collections::{HashMap, HashSet};

use vecto_rs::{linear::{Vector, VectorTrait, Mat4}, trig::to_radians};
use winit::event::{VirtualKeyCode, ElementState};

use crate::prospect_camera::ProspectCamera;

pub struct CameraController
{
    keys_down : HashSet<VirtualKeyCode>,
    mouse_down : bool,
    mouse_down_pos : Vector,
    pub units_per_second : f32
}

impl CameraController 
{
    pub fn new() -> Self
    {
        Self
        {
            keys_down : HashSet::new(),
            mouse_down : false,
            mouse_down_pos : Vector::default(),
            units_per_second : 3.
        }
    }

    pub fn process(&mut self, delta : f32, camera : &mut ProspectCamera)
    {
        let mut move_vector = Vector::new3(0., 0., 0.);

        if self.keys_down.contains(&VirtualKeyCode::W)
        {
            move_vector.z += self.units_per_second;
        }

        if self.keys_down.contains(&VirtualKeyCode::S)
        {
            move_vector.z -= self.units_per_second;
        }

        if self.keys_down.contains(&VirtualKeyCode::A)
        {
            move_vector.x -= self.units_per_second;
        }
        
        if self.keys_down.contains(&VirtualKeyCode::D)
        {
            move_vector.x += self.units_per_second;
        }

        if self.keys_down.contains(&VirtualKeyCode::LControl)
        {
            move_vector.y -= self.units_per_second;
        }
        
        if self.keys_down.contains(&VirtualKeyCode::Space)
        {
            move_vector.y += self.units_per_second;
        }
        
        if self.keys_down.contains(&VirtualKeyCode::Right)
        {
            camera.rotation.y += self.units_per_second * delta;
        }

        if self.keys_down.contains(&VirtualKeyCode::Left)
        {
            camera.rotation.y -= self.units_per_second * delta;
        }

        let x = move_vector.x * camera.rotation.y.cos() + move_vector.z * camera.rotation.y.sin();
        let z = -move_vector.x * camera.rotation.y.sin() + move_vector.z * camera.rotation.y.cos();

        camera.eye += Vector::new3(x, move_vector.y, z) * delta;
    }

    pub fn mouse_event(&mut self, state : ElementState)
    {
        self.mouse_down =  state == ElementState::Pressed
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