use raylib::prelude::*;
use crate::shader::{BodyType, ShaderManager};

pub struct CelestialBody {
    pub name: String,
    pub position: Vector3,
    pub radius: f32,
    pub color: Color,
    pub orbital_radius: f32,
    pub orbital_speed: f32,
    pub rotation_speed: f32,
    pub rotation_angle: f32,
    pub orbital_angle: f32,
    pub is_sun: bool,
    pub parent: Option<usize>,
    pub texture: Option<Texture2D>,
    pub body_type: BodyType,
}

impl CelestialBody {
    pub fn new_sun(name: &str, radius: f32, color: Color) -> Self {
        Self {
            name: name.to_string(),
            position: Vector3::zero(),
            radius,
            color,
            orbital_radius: 0.0,
            orbital_speed: 0.0,
            rotation_speed: 0.5,
            rotation_angle: 0.0,
            orbital_angle: 0.0,
            is_sun: true,
            parent: None,
            texture: None,
            body_type: BodyType::GasGiant,
        }
    }

    pub fn new_planet(
        name: &str,
        radius: f32,
        color: Color,
        orbital_radius: f32,
        orbital_speed: f32,
        rotation_speed: f32,
    ) -> Self {
        let body_type = BodyType::RockyPlanet;
        
        Self {
            name: name.to_string(),
            position: Vector3::new(orbital_radius, 0.0, 0.0),
            radius,
            color,
            orbital_radius,
            orbital_speed,
            rotation_speed,
            rotation_angle: 0.0,
            orbital_angle: 0.0,
            is_sun: false,
            parent: None,
            texture: None,
            body_type,
        }
    }

    pub fn new_moon(
        name: &str,
        radius: f32,
        color: Color,
        orbital_radius: f32,
        orbital_speed: f32,
        rotation_speed: f32,
        parent_index: usize,
    ) -> Self {
        let mut moon = Self::new_planet(name, radius, color, orbital_radius, orbital_speed, rotation_speed);
        moon.parent = Some(parent_index);
        moon.body_type = BodyType::Moon;
        moon.body_type = BodyType::Moon;
        moon
    }

    pub fn update(&mut self, delta_time: f32, parent_position: Option<Vector3>) {
        self.rotation_angle += self.rotation_speed * delta_time;
        if self.rotation_angle > 360.0 {
            self.rotation_angle -= 360.0;
        }

        if !self.is_sun {
            self.orbital_angle += self.orbital_speed * delta_time;
            if self.orbital_angle > 360.0 {
                self.orbital_angle -= 360.0;
            }

            let base_pos = parent_position.unwrap_or(Vector3::zero());
            
            self.position = Vector3::new(
                base_pos.x + self.orbital_radius * self.orbital_angle.to_radians().cos(),
                base_pos.y,
                base_pos.z + self.orbital_radius * self.orbital_angle.to_radians().sin(),
            );
        }
    }

    pub fn draw<D: RaylibDraw3D>(
        &self,
        d: &mut D,
        _model: &Model,
        _shader_manager: Option<&ShaderManager>,
        _time: f32,
    ) {
        d.draw_sphere(self.position, self.radius, self.color);
    }

    pub fn check_collision(&self, point: Vector3, safe_distance: f32) -> bool {
        let distance = (self.position - point).length();
        distance < (self.radius + safe_distance)
    }
}