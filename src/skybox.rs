use raylib::prelude::*;

pub struct Skybox {
    pub size: f32,
    pub model: Option<Model>,
    pub texture: Option<Texture2D>,
}

impl Skybox {
    pub fn new(size: f32, model: Option<Model>, texture: Option<Texture2D>) -> Self {
        Self { size, model, texture }
    }

    pub fn draw<D: RaylibDraw3D>(&self, d: &mut D, _camera_pos: Vector3) {
        // Dibuja esfera invertida con textura
        if let (Some(model), Some(texture)) = (&self.model, &self.texture) {
            // Aplicar textura al modelo
            unsafe {
                use raylib::consts::MaterialMapIndex;
                let materials = model.materials();
                if !materials.is_empty() {
                    let mat_ptr = materials.as_ptr() as *mut raylib::ffi::Material;
                    let maps_ptr = (*mat_ptr).maps;
                    (*maps_ptr.wrapping_add(MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize)).texture = **texture;
                }
            }
            
            // Dibujar esfera invertida fija en el origen (escala negativa en X para ver desde dentro)
            d.draw_model_ex(
                model,
                Vector3::zero(),  // Fijo en el origen, no sigue la cámara
                Vector3::new(0.0, 1.0, 0.0),
                0.0,
                Vector3::new(-self.size, self.size, self.size),
                Color::WHITE,
            );
        } else {
            // Fallback si no hay modelo o textura
            d.draw_cube_wires(Vector3::zero(), self.size, self.size, self.size, Color::new(30, 30, 60, 80));
        }
    }

    pub fn draw_stars<D: RaylibDraw3D>(&self, _d: &mut D, _camera_pos: Vector3) {
        // Ya no dibujamos estrellas procedurales, la textura las contiene
    }
}