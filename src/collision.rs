use raylib::prelude::*;
use crate::celestial_body::CelestialBody;

pub struct CollisionSystem {
    pub safe_distance: f32,
}

impl CollisionSystem {
    pub fn new(safe_distance: f32) -> Self {
        Self { safe_distance }
    }

    pub fn check_and_resolve(
        &self,
        position: Vector3,
        bodies: &[CelestialBody],
    ) -> Option<Vector3> {
        for body in bodies {
            if body.check_collision(position, self.safe_distance) {
                let direction = (position - body.position).normalized();
                let safe_pos = body.position + direction * (body.radius + self.safe_distance);
                return Some(safe_pos);
            }
        }
        None
    }

    pub fn is_colliding(&self, position: Vector3, bodies: &[CelestialBody]) -> bool {
        for body in bodies {
            if body.check_collision(position, self.safe_distance) {
                return true;
            }
        }
        false
    }
}