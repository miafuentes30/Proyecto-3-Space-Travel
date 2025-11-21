use raylib::prelude::*;

pub struct SoftwareRenderer {
	pub fov_deg: f32,
	pub near: f32,
	pub far: f32,
}

impl SoftwareRenderer {
	pub fn new() -> Self {
		Self { fov_deg: 60.0, near: 0.1, far: 2000.0 }
	}

	fn project(&self, point: Vector3, camera_pos: Vector3, camera_target: Vector3, screen_w: i32, screen_h: i32) -> Option<(i32, i32)> {
		let forward = (camera_target - camera_pos).normalized();
		let right = forward.cross(Vector3::new(0.0, 1.0, 0.0)).normalized();
		let up = right.cross(forward);
		let rel = point - camera_pos;
		let z = rel.dot(forward);
		if z < self.near || z > self.far { return None; }
		let x = rel.dot(right);
		let y = rel.dot(up);
		let fov_rad = (self.fov_deg.to_radians());
		let scale = (screen_h as f32 / 2.0) / (fov_rad * 0.5).tan();
		let sx = (screen_w as f32 / 2.0) + (x * scale / z);
		let sy = (screen_h as f32 / 2.0) - (y * scale / z);
		Some((sx as i32, sy as i32))
	}

	pub fn draw_bodies(&self, d2: &mut RaylibDrawHandle, bodies: &[crate::celestial_body::CelestialBody], camera_pos: Vector3, camera_target: Vector3) {
		let (w, h) = (d2.get_screen_width(), d2.get_screen_height());
		for body in bodies {
			if let Some((sx, sy)) = self.project(body.position, camera_pos, camera_target, w, h) {
				let radius_px = (body.radius * 50.0 / (1.0 + (body.position - camera_pos).length() * 0.02)) as i32;
				d2.draw_circle(sx, sy, radius_px as f32, body.color);
			}
		}
	}

	pub fn draw_orbits(&self, d2: &mut RaylibDrawHandle, bodies: &[crate::celestial_body::CelestialBody], camera_pos: Vector3, camera_target: Vector3) {
		let (w, h) = (d2.get_screen_width(), d2.get_screen_height());
		for body in bodies {
			if body.is_sun || body.parent.is_some() { continue; }
			let center = Vector3::zero();
			let mut points: Vec<(i32, i32)> = Vec::new();
			let segments = 60;
			for i in 0..segments {
				let a = (i as f32 / segments as f32) * std::f32::consts::TAU;
				let p = Vector3::new(center.x + body.orbital_radius * a.cos(), center.y, center.z + body.orbital_radius * a.sin());
				if let Some(pt) = self.project(p, camera_pos, camera_target, w, h) { points.push(pt); }
			}
			for wseg in points.windows(2) {
				if let [(x1, y1), (x2, y2)] = wseg { d2.draw_line(*x1, *y1, *x2, *y2, Color::new(100, 100, 100, 120)); }
			}
		}
	}
}
