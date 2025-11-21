use raylib::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyType {
    Star,
    RockyPlanet,
    GasGiant,
    Moon,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn dot(&self, other: Vec2) -> f32 { self.x * other.x + self.y * other.y }
    pub fn length(&self) -> f32 { (self.x * self.x + self.y * self.y).sqrt() }
    pub fn floor(&self) -> Vec2 { Vec2::new(self.x.floor(), self.y.floor()) }
    pub fn fract(&self) -> Vec2 { Vec2::new(self.x.fract(), self.y.fract()) }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 { Vec2::new(self.x + o.x, self.y + o.y) }
}
impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 { Vec2::new(self.x - o.x, self.y - o.y) }
}
impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f32) -> Vec2 { Vec2::new(self.x * s, self.y * s) }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn to_color(&self) -> Color {
        Color::new(
            (self.x.clamp(0.0, 1.0) * 255.0) as u8,
            (self.y.clamp(0.0, 1.0) * 255.0) as u8,
            (self.z.clamp(0.0, 1.0) * 255.0) as u8, 255)
    }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f32) -> Vec3 { Vec3::new(self.x * s, self.y * s, self.z * s) }
}
impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, o: Vec3) -> Vec3 { Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z) }
}

// ===== FUNCIONES DE RUIDO =====
fn hash(p: Vec2) -> f32 { (p.dot(Vec2::new(127.1, 311.7)).sin() * 43758.5453).fract() }

fn noise(p: Vec2) -> f32 {
    let i = p.floor();
    let f = p.fract();
    let f = Vec2::new(f.x * f.x * (3.0 - 2.0 * f.x), f.y * f.y * (3.0 - 2.0 * f.y));
    let a = hash(i); let b = hash(i + Vec2::new(1.0, 0.0));
    let c = hash(i + Vec2::new(0.0, 1.0)); let d = hash(i + Vec2::new(1.0, 1.0));
    a*(1.0-f.x)*(1.0-f.y) + b*f.x*(1.0-f.y) + c*(1.0-f.x)*f.y + d*f.x*f.y
}

// Ruido con mezcla en zona de la costura vertical (u=0/1) para evitar línea
fn seam_noise(p: Vec2) -> f32 {
    let base = noise(p);
    let blend = 0.03; // ancho de zona de mezcla
    if p.x < blend {
        let other = noise(Vec2::new(p.x + 1.0, p.y));
        let t = smoothstep(0.0, blend, p.x);
        mix_f32(other, base, t)
    } else if p.x > 1.0 - blend {
        let other = noise(Vec2::new(p.x - 1.0, p.y));
        let t = smoothstep(1.0 - blend, 1.0, p.x);
        mix_f32(base, other, t)
    } else {
        base
    }
}

fn fbm(mut p: Vec2, oct: i32) -> f32 {
    let (mut v, mut a) = (0.0, 0.5);
    for _ in 0..oct { v += a * seam_noise(p); p = p * 2.0; a *= 0.5; }
    v
}

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn mix_f32(a: f32, b: f32, t: f32) -> f32 { a * (1.0 - t) + b * t }
fn mix_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    Vec3::new(mix_f32(a.x, b.x, t), mix_f32(a.y, b.y, t), mix_f32(a.z, b.z, t))
}

// ===== SHADERS =====

/// SOL - Gaseoso amarillo-naranja con turbulencia y plasma
fn sol_shader(uv: Vec2, time: f32) -> Vec3 {
    let turb1 = fbm(Vec2::new(uv.x * 4.0 + time * 0.08, uv.y * 4.0 - time * 0.05), 4);
    let turb2 = fbm(Vec2::new(uv.x * 8.0 - time * 0.12, uv.y * 8.0 + time * 0.1), 3);
    let band_distort = turb1 * 1.5 + turb2 * 0.8;
    let bands = ((uv.y * 8.0 + band_distort) * 3.14159).sin() * 0.5 + 0.5;
    let cx = uv.x - 0.5; let cy = uv.y - 0.5;
    let angle = cy.atan2(cx); let dist = (cx*cx + cy*cy).sqrt();
    let swirl = noise(Vec2::new(angle * 2.0 + time * 0.1, dist * 10.0 + turb1 * 3.0));
    let plasma = fbm(Vec2::new(uv.x * 6.0 + swirl * 2.0 + time * 0.15, uv.y * 6.0 + turb2 * 2.0 - time * 0.1), 5);
    let color_mix = (bands * 0.3 + plasma * 0.4 + swirl * 0.3).clamp(0.0, 1.0);
    
    let c1 = Vec3::new(1.0, 0.95, 0.3); let c2 = Vec3::new(1.0, 0.8, 0.1);
    let c3 = Vec3::new(1.0, 0.5, 0.05); let c4 = Vec3::new(0.95, 0.3, 0.02);
    let color = if color_mix < 0.25 { mix_vec3(c1, c2, color_mix / 0.25) }
        else if color_mix < 0.5 { mix_vec3(c2, c3, (color_mix - 0.25) / 0.25) }
        else if color_mix < 0.75 { mix_vec3(c3, c4, (color_mix - 0.5) / 0.25) }
        else { mix_vec3(c4, c3, (color_mix - 0.75) / 0.25) };
    let glow = 1.0 - smoothstep(0.0, 0.8, dist * 1.5);
    let mut fc = color * (0.85 + glow * 0.25);
    let flare = noise(Vec2::new(uv.x * 20.0 + time * 0.3, uv.y * 20.0 - time * 0.2));
    if flare > 0.7 { fc = fc * (1.0 + (flare - 0.7) * 2.0); }
    fc
}

/// MERCURIO - Agua/líquido metálico gris con ondas
fn mercurio_shader(uv: Vec2, time: f32) -> Vec3 {
    let cx = uv.x - 0.5; let cy = uv.y - 0.5;
    let dist = (cx*cx + cy*cy).sqrt();
    let wave1 = ((dist * 25.0 - time * 2.0).sin() * 0.5 + 0.5) * (1.0 - dist).max(0.0);
    let d2 = ((uv.x - 0.3).powi(2) + (uv.y - 0.7).powi(2)).sqrt();
    let wave2 = ((d2 * 20.0 - time * 1.5).sin() * 0.5 + 0.5) * (1.0 - d2).max(0.0) * 0.6;
    let dir1 = ((uv.x * 8.0 + uv.y * 4.0 - time * 1.2).sin() * 0.5 + 0.5) * 0.4;
    let dir2 = ((uv.x * 6.0 - uv.y * 10.0 + time * 0.8).sin() * 0.5 + 0.5) * 0.3;
    let perturb = fbm(Vec2::new(uv.x * 5.0 + time * 0.2, uv.y * 5.0 - time * 0.15), 3);
    let waves = wave1 + wave2 + dir1 + dir2;
    let water = (waves * 0.6 + perturb * 0.4).clamp(0.0, 1.0);
    let spec = noise(Vec2::new(uv.x * 30.0 + waves * 5.0 + time * 0.5, uv.y * 30.0 + perturb * 3.0));
    let specular = smoothstep(0.65, 0.85, spec) * 0.6;
    let caust1 = noise(Vec2::new(uv.x * 15.0 + time * 0.3, uv.y * 15.0 + time * 0.2));
    let caust2 = noise(Vec2::new(uv.x * 15.0 - time * 0.25, uv.y * 15.0 - time * 0.3));
    let caustics = (caust1 * caust2 * 4.0).clamp(0.0, 1.0) * 0.25;
    
    let g1 = Vec3::new(0.25, 0.28, 0.32); let g2 = Vec3::new(0.45, 0.48, 0.52);
    let g3 = Vec3::new(0.65, 0.68, 0.72);
    let mut color = if water < 0.5 { mix_vec3(g1, g2, water / 0.5) } else { mix_vec3(g2, g3, (water - 0.5) / 0.5) };
    color = color + Vec3::new(specular, specular, specular * 1.1);
    color = color + Vec3::new(caustics * 0.8, caustics * 0.9, caustics);
    color
}

/// VENUS - Atmósfera densa naranja/amarilla con nubes tóxicas
fn venus_shader(uv: Vec2, time: f32) -> Vec3 {
    let clouds1 = fbm(Vec2::new(uv.x * 3.0 + time * 0.05, uv.y * 4.0 + time * 0.02), 5);
    let clouds2 = fbm(Vec2::new(uv.x * 6.0 - time * 0.03, uv.y * 5.0 - time * 0.04), 4);
    let swirl = noise(Vec2::new(uv.x * 2.0 + clouds1 * 2.0, uv.y * 2.0 + time * 0.01));
    let atmosphere = (clouds1 * 0.5 + clouds2 * 0.3 + swirl * 0.2).clamp(0.0, 1.0);
    let bands = ((uv.y * 12.0 + clouds1 * 3.0).sin() * 0.5 + 0.5) * 0.3;
    
    let c1 = Vec3::new(0.95, 0.75, 0.3); let c2 = Vec3::new(0.85, 0.55, 0.15);
    let c3 = Vec3::new(0.7, 0.4, 0.1);
    let mix_val = (atmosphere + bands).clamp(0.0, 1.0);
    let color = if mix_val < 0.5 { mix_vec3(c1, c2, mix_val / 0.5) } else { mix_vec3(c2, c3, (mix_val - 0.5) / 0.5) };
    color * 0.95
}

/// TIERRA - Océanos y continentes rocosos
fn tierra_shader(uv: Vec2, _time: f32) -> Vec3 {
    // Usar múltiples octavas de ruido para continentes irregulares
    let cont1 = fbm(Vec2::new(uv.x * 3.5 + 0.5, uv.y * 3.5 + 0.3), 6);
    let cont2 = noise(Vec2::new(uv.x * 7.0, uv.y * 7.0)) * 0.3;
    let cont = cont1 + cont2 * 0.5;
    
    let ocean_deep = Vec3::new(0.05, 0.15, 0.4);
    let ocean_shallow = Vec3::new(0.1, 0.25, 0.55);
    let land_low = Vec3::new(0.2, 0.45, 0.12);
    let land_mid = Vec3::new(0.35, 0.4, 0.15);
    let land_high = Vec3::new(0.5, 0.42, 0.25);
    let mountain = Vec3::new(0.4, 0.35, 0.3);
    let snow = Vec3::new(0.92, 0.92, 0.95);
    
    let mut color = if cont < 0.4 {
        // Océano con variación de profundidad
        let depth_var = noise(Vec2::new(uv.x * 12.0, uv.y * 12.0));
        mix_vec3(ocean_deep, ocean_shallow, depth_var)
    } else if cont < 0.5 {
        // Costa/tierra baja
        let t = (cont - 0.4) / 0.1;
        mix_vec3(ocean_shallow, land_low, t)
    } else if cont < 0.65 {
        // Tierra media con textura
        let detail = noise(Vec2::new(uv.x * 20.0, uv.y * 20.0)) * 0.15;
        let t = (cont - 0.5) / 0.15;
        mix_vec3(land_low, land_mid, t) * (0.9 + detail)
    } else if cont < 0.8 {
        // Tierra alta/montañas
        let rock = noise(Vec2::new(uv.x * 25.0, uv.y * 25.0)) * 0.2;
        let t = (cont - 0.65) / 0.15;
        mix_vec3(land_high, mountain, t) * (0.85 + rock)
    } else {
        // Picos nevados
        let snow_var = noise(Vec2::new(uv.x * 30.0, uv.y * 30.0)) * 0.1;
        let t = (cont - 0.8) / 0.2;
        mix_vec3(mountain, snow, t) * (0.95 + snow_var)
    };
    
    // Casquetes polares
    let lat = (uv.y - 0.5).abs() * 2.0;
    if lat > 0.82 {
        let polar = (lat - 0.82) / 0.18;
        let ice = Vec3::new(0.88, 0.9, 0.95);
        color = mix_vec3(color, ice, polar * 0.8);
    }
    color
}

/// MARTE - Desierto rocoso rojo con cráteres y variación
fn marte_shader(uv: Vec2, _time: f32) -> Vec3 {
    // Terreno base con múltiples capas de ruido
    let terrain1 = fbm(Vec2::new(uv.x * 4.0, uv.y * 4.0), 6);
    let terrain2 = noise(Vec2::new(uv.x * 8.0 + 2.0, uv.y * 8.0 + 1.5)) * 0.4;
    let terrain3 = noise(Vec2::new(uv.x * 16.0, uv.y * 16.0)) * 0.2;
    let terrain = terrain1 + terrain2 * 0.5 + terrain3 * 0.3;
    
    // Cráteres de diferentes tamaños
    let crater1 = noise(Vec2::new(uv.x * 10.0, uv.y * 10.0));
    let crater2 = noise(Vec2::new(uv.x * 20.0 + 5.0, uv.y * 20.0 + 3.0));
    let crater3 = noise(Vec2::new(uv.x * 35.0, uv.y * 35.0));
    let craters = smoothstep(0.55, 0.65, crater1) * 0.25 
                + smoothstep(0.58, 0.68, crater2) * 0.15
                + smoothstep(0.6, 0.7, crater3) * 0.1;
    
    // Paleta de colores marcianos
    let c_light = Vec3::new(0.85, 0.45, 0.2);    // Naranja claro
    let c_mid = Vec3::new(0.7, 0.32, 0.12);      // Rojo óxido
    let c_dark = Vec3::new(0.5, 0.22, 0.08);     // Rojo oscuro
    let c_shadow = Vec3::new(0.35, 0.15, 0.05);  // Sombra de cráteres
    
    // Mezcla basada en elevación
    let elev = (terrain * 0.7).clamp(0.0, 1.0);
    let mut color = if elev < 0.35 {
        mix_vec3(c_dark, c_mid, elev / 0.35)
    } else if elev < 0.7 {
        mix_vec3(c_mid, c_light, (elev - 0.35) / 0.35)
    } else {
        let rock_detail = noise(Vec2::new(uv.x * 40.0, uv.y * 40.0)) * 0.1;
        c_light * (0.95 + rock_detail)
    };
    
    // Aplicar cráteres (oscurecen el terreno)
    color = color * (1.0 - craters);
    color = mix_vec3(color, c_shadow, craters * 0.6);
    
    // Textura de polvo/arena fina
    let dust = noise(Vec2::new(uv.x * 50.0, uv.y * 50.0)) * 0.08;
    color = color * (0.96 + dust);
    
    // Casquetes polares pequeños
    let lat = (uv.y - 0.5).abs() * 2.0;
    if lat > 0.9 {
        let ice = Vec3::new(0.9, 0.88, 0.85);
        let polar = (lat - 0.9) / 0.1;
        color = mix_vec3(color, ice, polar * 0.7);
    }
    
    color
}

/// JUPITER - Bandas y Gran Mancha Roja
fn jupiter_shader(uv: Vec2, time: f32) -> Vec3 {
    let band_y = uv.y + fbm(Vec2::new(uv.x * 8.0 + time * 0.02, uv.y * 2.0), 3) * 0.08;
    let band = ((band_y * 25.0).sin() * 0.5 + 0.5);
    let turb = fbm(Vec2::new(uv.x * 12.0 + time * 0.03, uv.y * 8.0), 4) * 0.2;
    let c1 = Vec3::new(0.85, 0.75, 0.6); let c2 = Vec3::new(0.7, 0.55, 0.4);
    let c3 = Vec3::new(0.55, 0.4, 0.3);
    let mix_val = (band + turb).clamp(0.0, 1.0);
    let mut color = if mix_val < 0.5 { mix_vec3(c1, c2, mix_val / 0.5) } else { mix_vec3(c2, c3, (mix_val - 0.5) / 0.5) };
    // Gran Mancha Roja
    let spot_x = uv.x - 0.65; let spot_y = uv.y - 0.55;
    let spot_dist = (spot_x * spot_x * 4.0 + spot_y * spot_y * 16.0).sqrt();
    if spot_dist < 0.15 {
        let spot_color = Vec3::new(0.8, 0.3, 0.2);
        let swirl = noise(Vec2::new(spot_x * 30.0 + time * 0.5, spot_y * 30.0)) * 0.2;
        color = mix_vec3(color, spot_color * (1.0 + swirl), 1.0 - spot_dist / 0.15);
    }
    color
}

/// SATURNO - Bandas suaves beige/dorado
fn saturno_shader(uv: Vec2, time: f32) -> Vec3 {
    let band_y = uv.y + noise(Vec2::new(uv.x * 6.0 + time * 0.015, uv.y * 3.0)) * 0.06;
    let band = ((band_y * 20.0).sin() * 0.5 + 0.5);
    let turb = fbm(Vec2::new(uv.x * 8.0 + time * 0.02, uv.y * 6.0), 3) * 0.15;
    let c1 = Vec3::new(0.95, 0.88, 0.7); let c2 = Vec3::new(0.85, 0.75, 0.55);
    let c3 = Vec3::new(0.75, 0.65, 0.45);
    let mix_val = (band + turb).clamp(0.0, 1.0);
    if mix_val < 0.5 { mix_vec3(c1, c2, mix_val / 0.5) } else { mix_vec3(c2, c3, (mix_val - 0.5) / 0.5) }
}

/// URANO - Azul-verdoso uniforme con ligeras bandas
fn urano_shader(uv: Vec2, time: f32) -> Vec3 {
    let band = ((uv.y * 15.0 + time * 0.01).sin() * 0.5 + 0.5) * 0.1;
    let atm = fbm(Vec2::new(uv.x * 4.0 + time * 0.01, uv.y * 4.0), 3) * 0.1;
    let c1 = Vec3::new(0.6, 0.85, 0.9); let c2 = Vec3::new(0.5, 0.75, 0.82);
    mix_vec3(c1, c2, (band + atm).clamp(0.0, 1.0))
}

/// NEPTUNO - Azul intenso con tormentas
fn neptuno_shader(uv: Vec2, time: f32) -> Vec3 {
    let band = ((uv.y * 18.0 + time * 0.02).sin() * 0.5 + 0.5);
    let turb = fbm(Vec2::new(uv.x * 10.0 + time * 0.04, uv.y * 8.0), 4) * 0.2;
    let c1 = Vec3::new(0.2, 0.4, 0.9); let c2 = Vec3::new(0.15, 0.3, 0.75);
    let c3 = Vec3::new(0.1, 0.2, 0.6);
    let mix_val = (band * 0.5 + turb).clamp(0.0, 1.0);
    let mut color = if mix_val < 0.5 { mix_vec3(c1, c2, mix_val / 0.5) } else { mix_vec3(c2, c3, (mix_val - 0.5) / 0.5) };
    // Gran Mancha Oscura
    let spot_x = uv.x - 0.4; let spot_y = uv.y - 0.45;
    let spot_dist = (spot_x * spot_x * 3.0 + spot_y * spot_y * 12.0).sqrt();
    if spot_dist < 0.12 { color = mix_vec3(color, c3 * 0.7, 1.0 - spot_dist / 0.12); }
    color
}

/// LUNA - Cráteres grises
fn moon_shader(uv: Vec2, moon_color: Vec3, _time: f32) -> Vec3 {
    let cr1 = smoothstep(0.5, 0.6, noise(uv * 10.0)) * 0.4;
    let cr2 = smoothstep(0.55, 0.65, noise(uv * 25.0)) * 0.2;
    let cr3 = smoothstep(0.6, 0.7, noise(uv * 40.0)) * 0.1;
    let surf = noise(uv * 12.0) * 0.15;
    let mut color = moon_color;
    color = color * (1.0 - cr1 - cr2 - cr3);
    color = color * (1.0 + surf) * 0.6;
    color
}

// ===== SHADER MANAGER =====
pub struct ShaderManager { textures: HashMap<BodyType, Image> }

impl ShaderManager {
    pub fn new(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Self {
        println!("\n🔄 Cargando shaders CPU...");
        println!("✅ Shader Manager iniciado\n");
        ShaderManager { textures: HashMap::new() }
    }

    pub fn render_to_image(&mut self, body_type: BodyType, planet_color: Color, time: f32, size: i32, name: &str) -> Image {
        let mut img = Image::gen_image_color(size, size, Color::BLANK);
        let cv = Vec3::new(planet_color.r as f32 / 255.0, planet_color.g as f32 / 255.0, planet_color.b as f32 / 255.0);

        for y in 0..size {
            for x in 0..size {
                let uv = Vec2::new(x as f32 / size as f32, y as f32 / size as f32);
                let pixel = match name {
                    "Sol" => sol_shader(uv, time),
                    "Mercurio" => mercurio_shader(uv, time),
                    "Venus" => venus_shader(uv, time),
                    "Tierra" => tierra_shader(uv, time),
                    "Marte" => marte_shader(uv, time),
                    "Jupiter" => jupiter_shader(uv, time),
                    "Saturno" => saturno_shader(uv, time),
                    "Urano" => urano_shader(uv, time),
                    "Neptuno" => neptuno_shader(uv, time),
                    _ => match body_type {
                        BodyType::Star => sol_shader(uv, time),
                        BodyType::Moon => moon_shader(uv, cv, time),
                        BodyType::GasGiant => saturno_shader(uv, time),
                        BodyType::RockyPlanet => moon_shader(uv, cv, time),
                    }
                };
                img.draw_pixel(x, y, pixel.to_color());
            }
        }
        img
    }

    pub fn create_texture_for_body(&mut self, body_type: BodyType, planet_color: Color, time: f32) -> Image {
        self.render_to_image(body_type, planet_color, time, 128, "")
    }

    pub fn create_texture_for_body_named(&mut self, body_type: BodyType, planet_color: Color, time: f32, name: &str) -> Image {
        self.render_to_image(body_type, planet_color, time, 128, name)
    }
}