use raylib::prelude::*;

pub struct Spaceship {
    pub offset_from_camera: Vector3,
    pub scale: f32,
    pub rotation: Vector3,
    pub color: Color,
    pub orbit_demo: bool,
    pub orbit_angle: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub yaw_offset_deg: f32,
    pub tilt_deg: f32,
}

impl Spaceship {
    pub fn new() -> Self {
        Self {
            offset_from_camera: Vector3::new(1.5, -0.8, 4.0),
            scale: 0.25,
            rotation: Vector3::zero(),
            color: Color::MAGENTA,
            orbit_demo: true,
            orbit_angle: 0.0,
            orbit_radius: 180.0,
            orbit_speed: 0.35,
            yaw_offset_deg: 90.0,
            tilt_deg: 15.0,
        }
    }

    pub fn get_position(&self, camera_pos: Vector3, camera_target: Vector3) -> Vector3 {
        if self.orbit_demo {
            // Órbita vertical inclinada 15° alrededor del origen
            let a = self.orbit_angle;
            let y = self.orbit_radius * a.sin();
            let z = self.orbit_radius * a.cos();
            let t = self.tilt_deg.to_radians();
            let y2 = y * t.cos() - z * t.sin();
            let z2 = y * t.sin() + z * t.cos();
            return Vector3::new(0.0, y2, z2);
        }
        let forward = (camera_target - camera_pos).normalized();
        let right = forward.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();
        let up = right.cross(forward);
        camera_pos + forward * self.offset_from_camera.z + right * self.offset_from_camera.x + up * self.offset_from_camera.y
    }

    pub fn draw<D: RaylibDraw3D>(&self, d: &mut D, model: &Model, camera_pos: Vector3, camera_target: Vector3) {
        let position = self.get_position(camera_pos, camera_target);
        // Dirección para la orientación
        let forward = if self.orbit_demo {
            // Tangente de la órbita inclinada en YZ
            let a = self.orbit_angle;
            let dy = self.orbit_radius * a.cos();
            let dz = -self.orbit_radius * a.sin();
            let t = self.tilt_deg.to_radians();
            let dy2 = dy * t.cos() - dz * t.sin();
            let dz2 = dy * t.sin() + dz * t.cos();
            Vector3::new(0.0, dy2, dz2).normalized()
        } else {
            (camera_target - camera_pos).normalized()
        };
        let yaw = forward.z.atan2(forward.x); // rad
        let yaw_degrees = yaw.to_degrees();
        d.draw_model_ex(
            model,
            position,
            Vector3::new(0.0, 1.0, 0.0),
            -yaw_degrees + self.yaw_offset_deg,
            Vector3::new(self.scale, self.scale, self.scale),
            self.color,
        );
        // Indicador frontal
        let nose = position + forward * (self.scale * 2.0);
        d.draw_line_3D(position, nose, Color::SKYBLUE);
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.orbit_demo {
            self.orbit_angle += self.orbit_speed * delta_time;
            if self.orbit_angle > std::f32::consts::TAU { self.orbit_angle -= std::f32::consts::TAU; }
        }
    }

    pub fn toggle_orbit_demo(&mut self) { self.orbit_demo = !self.orbit_demo; }
}