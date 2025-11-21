use raylib::prelude::*;

pub struct WarpEffect {
    pub particle_count: i32,
    pub particles: Vec<WarpParticle>,
    pub active: bool,
    pub progress: f32,
}

struct WarpParticle {
    position: Vector3,
    velocity: Vector3,
    lifetime: f32,
    max_lifetime: f32,
}

impl WarpEffect {
    pub fn new() -> Self {
        Self {
            particle_count: 0,  // Sin partículas visibles
            particles: Vec::new(),
            active: false,
            progress: 0.0,
        }
    }

    pub fn start(&mut self, _start_pos: Vector3, _end_pos: Vector3) {
        // Desactivamos las partículas para evitar la esfera visible
        self.particles.clear();
        self.active = true;
        self.progress = 0.0;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.active {
            self.progress += delta_time * 2.0;
            if self.progress >= 1.0 {
                self.active = false;
                self.progress = 0.0;
            }
        }
        
        // Limpiar partículas viejas si las hay
        self.particles.retain_mut(|particle| {
            particle.lifetime += delta_time;
            particle.position = particle.position + particle.velocity * delta_time;
            particle.lifetime < particle.max_lifetime
        });
    }

    pub fn draw<D: RaylibDraw3D>(&self, _d: &mut D) {
        // No dibujamos nada - el efecto visual está desactivado
        // Si quieres un efecto más sutil, puedes agregar líneas de velocidad aquí
    }

    pub fn is_active(&self) -> bool {
        self.active || !self.particles.is_empty()
    }
}