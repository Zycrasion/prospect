use std::{collections::{HashMap, HashSet}, env::consts::FAMILY};

use vecto_rs::{linear::{Vector, VectorTrait, Mat4}, trig::to_radians};
use winit::{event::{VirtualKeyCode, ElementState}, dpi::{PhysicalPosition, LogicalPosition}, window};

use crate::{prospect_camera::ProspectCamera, abstraction::prospect_window::ProspectWindow};

pub struct CameraController
{
    keys_down : HashSet<VirtualKeyCode>,
    mouse_down : bool,
    mouse_down_pos  : Vector,
    current_mouse_pos : Vector,
    drag_amount : Vector,
    pub units_per_second : f32,
    pub sprint_multiplier : f32,
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
            current_mouse_pos : Vector::default(),
            drag_amount : Vector::default(),
            units_per_second : 3.,
            sprint_multiplier : 5.
        }
    }

    pub fn process(&mut self, delta : f32, camera : &mut ProspectCamera, window : &mut ProspectWindow)
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

        camera.rotation.y += to_radians(self.drag_amount.x) * 2.;
        camera.rotation.x += to_radians(self.drag_amount.y);
        camera.rotation.x = camera.rotation.x.clamp(to_radians(-45.), to_radians(45.));
        self.drag_amount = Vector::default();

        if self.keys_down.contains(&VirtualKeyCode::LShift)
        {
            move_vector *= self.sprint_multiplier;
        }

        move_vector.rotate_x(camera.rotation.x);
        move_vector.rotate_y(camera.rotation.y);
        
        if self.mouse_down
        {
            let _ = window.get_window().set_cursor_position(LogicalPosition::new(self.mouse_down_pos.x, self.mouse_down_pos.y));                
        }
        camera.eye += move_vector * delta;
    }

    pub fn mouse_click_event(&mut self, state : ElementState, window : &mut ProspectWindow)
    {
        if state == ElementState::Pressed
        {
            window.get_window().set_cursor_visible(self.mouse_down);
            self.mouse_down = !self.mouse_down;
            self.mouse_down_pos = self.current_mouse_pos;                
        }
    }

    pub fn mouse_move_event(&mut self, pos : Vector, window : &mut ProspectWindow)
    {
        if self.mouse_down
        {
            let _ = window.get_window().set_cursor_position(LogicalPosition::new(self.mouse_down_pos.x, self.mouse_down_pos.y));
        } else
        {
            self.current_mouse_pos = pos;
            self.drag_amount = Vector::default();
        }
    }

    pub fn mouse_delta(&mut self, delta : Vector)
    {
        if self.mouse_down
        {
            self.drag_amount = delta;
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