use raylib::prelude::*;

pub struct OrbitRenderer {
    pub segments: i32,
}

impl OrbitRenderer {
    pub fn new() -> Self {
        Self { segments: 100 }
    }

    pub fn draw_orbit<D: RaylibDraw3D>(
        &self,
        d: &mut D,
        center: Vector3,
        radius: f32,
        color: Color,
    ) {
        if radius < 0.1 {
            return;
        }

        let angle_step = 360.0 / self.segments as f32;
        
        for i in 0..self.segments {
            let angle1 = (i as f32 * angle_step).to_radians();
            let angle2 = ((i + 1) as f32 * angle_step).to_radians();

            let point1 = Vector3::new(
                center.x + radius * angle1.cos(),
                center.y,
                center.z + radius * angle1.sin(),
            );

            let point2 = Vector3::new(
                center.x + radius * angle2.cos(),
                center.y,
                center.z + radius * angle2.sin(),
            );

            d.draw_line_3D(point1, point2, color);
        }
    }
}