use raylib::prelude::*;

pub struct CameraController {
    pub camera: Camera3D,
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub target_position: Option<Vector3>,
    pub warp_progress: f32,
    pub is_warping: bool,
    pub start_warp_pos: Vector3,
    warp_offset: Vector3,
    warp_target_body_pos: Vector3,
}

impl CameraController {
    pub fn new(position: Vector3) -> Self {
        let target = Vector3::new(0.0, 0.0, 0.0);
        Self {
            camera: Camera3D::perspective(
                position,
                target,
                Vector3::new(0.0, 1.0, 0.0),
                60.0,
            ),
            move_speed: 10.0,
            rotation_speed: 50.0,
            target_position: None,
            warp_progress: 0.0,
            is_warping: false,
            start_warp_pos: position,
            warp_offset: target - position,
            warp_target_body_pos: Vector3::zero(),
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, delta_time: f32) {
        if self.is_warping {
            self.update_warp(delta_time);
            return;
        }

        let forward = (self.camera.target - self.camera.position).normalized();
        let right = forward.cross(self.camera.up).normalized();

        // WASD: movimiento horizontal
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.camera.position.x += forward.x * self.move_speed * delta_time;
            self.camera.position.z += forward.z * self.move_speed * delta_time;
            self.camera.target.x += forward.x * self.move_speed * delta_time;
            self.camera.target.z += forward.z * self.move_speed * delta_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.camera.position.x -= forward.x * self.move_speed * delta_time;
            self.camera.position.z -= forward.z * self.move_speed * delta_time;
            self.camera.target.x -= forward.x * self.move_speed * delta_time;
            self.camera.target.z -= forward.z * self.move_speed * delta_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.camera.position.x -= right.x * self.move_speed * delta_time;
            self.camera.position.z -= right.z * self.move_speed * delta_time;
            self.camera.target.x -= right.x * self.move_speed * delta_time;
            self.camera.target.z -= right.z * self.move_speed * delta_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.camera.position.x += right.x * self.move_speed * delta_time;
            self.camera.position.z += right.z * self.move_speed * delta_time;
            self.camera.target.x += right.x * self.move_speed * delta_time;
            self.camera.target.z += right.z * self.move_speed * delta_time;
        }

        // Espacio/Shift: arriba/abajo
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            self.camera.position.y += self.move_speed * delta_time;
            self.camera.target.y += self.move_speed * delta_time;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            self.camera.position.y -= self.move_speed * delta_time;
            self.camera.target.y -= self.move_speed * delta_time;
        }

        // Rotacion con flechas
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            let angle = -self.rotation_speed * delta_time * std::f32::consts::PI / 180.0;
            self.rotate_camera(angle);
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            let angle = self.rotation_speed * delta_time * std::f32::consts::PI / 180.0;
            self.rotate_camera(angle);
        }
    }

    fn rotate_camera(&mut self, angle: f32) {
        let direction = self.camera.target - self.camera.position;
        let rotated = Vector3::new(
            direction.x * angle.cos() - direction.z * angle.sin(),
            direction.y,
            direction.x * angle.sin() + direction.z * angle.cos(),
        );
        self.camera.target = self.camera.position + rotated;
    }

    pub fn start_warp(&mut self, target: Vector3) {
        self.is_warping = true;
        self.warp_progress = 0.0;
        self.start_warp_pos = self.camera.position;
        self.target_position = Some(target);
        self.warp_target_body_pos = target - Vector3::new(0.0, 5.0, 15.0);
    }

    fn update_warp(&mut self, delta_time: f32) {
        self.warp_progress += delta_time * 2.0;

        if self.warp_progress >= 1.0 {
            self.warp_progress = 1.0;
            self.is_warping = false;
            if let Some(target) = self.target_position {
                self.camera.position = target;
                self.camera.target = self.warp_target_body_pos;
            }
        } else if let Some(target) = self.target_position {
            let t = self.warp_progress;
            let smooth_t = t * t * (3.0 - 2.0 * t);
            
            self.camera.position = Vector3::new(
                self.start_warp_pos.x + (target.x - self.start_warp_pos.x) * smooth_t,
                self.start_warp_pos.y + (target.y - self.start_warp_pos.y) * smooth_t,
                self.start_warp_pos.z + (target.z - self.start_warp_pos.z) * smooth_t,
            );
            let start_target = self.camera.target;
            self.camera.target = Vector3::new(
                start_target.x + (self.warp_target_body_pos.x - start_target.x) * smooth_t * 0.5,
                start_target.y + (self.warp_target_body_pos.y - start_target.y) * smooth_t * 0.5,
                start_target.z + (self.warp_target_body_pos.z - start_target.z) * smooth_t * 0.5,
            );
        }
    }

    pub fn apply_collision(&mut self, collision_point: Vector3, safe_distance: f32) {
        let direction = (self.camera.position - collision_point).normalized();
        let new_pos = collision_point + direction * safe_distance;
        
        let view_dir = self.camera.target - self.camera.position;
        self.camera.position = new_pos;
        self.camera.target = new_pos + view_dir;
    }
}