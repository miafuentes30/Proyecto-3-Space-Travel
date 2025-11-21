mod camera;
mod celestial_body;
mod spaceship;
mod skybox;
mod orbit;
mod collision;
mod warp_effect;
mod shader;

use raylib::prelude::*;
use camera::CameraController;
use celestial_body::CelestialBody;
use spaceship::Spaceship;
use skybox::Skybox;
use orbit::OrbitRenderer;
use collision::CollisionSystem;
use warp_effect::WarpEffect;
use shader::{ShaderManager, BodyType};

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Space Travel - Sistema Solar")
        .build();

    rl.set_target_fps(60);

    let mut sphere_model = rl.load_model(&thread, "assets/models/sphere.obj")
        .expect("No se pudo cargar sphere.obj");
    let ship_model = rl.load_model(&thread, "assets/models/nave.obj")
        .expect("No se pudo cargar nave.obj");
    let sky_tex = rl.load_texture(&thread, "assets/textures/skybox.png").ok();
    let sky_model = rl.load_model(&thread, "assets/models/sphere.obj").ok();

    let mut camera_controller = CameraController::new(Vector3::new(0.0, 30.0, 50.0));
    let mut spaceship = Spaceship::new();
    let skybox = Skybox::new(1000.0, sky_model, sky_tex);
    let orbit_renderer = OrbitRenderer::new();
    let collision_system = CollisionSystem::new(2.0);
    let mut warp_effect = WarpEffect::new();
    let mut shader_manager = ShaderManager::new(&mut rl, &thread);

    let mut celestial_bodies = create_solar_system();

    // Variables de estado
    let mut show_orbits = true;
    let mut show_info = true;
    let mut top_down = false;
    let mut elapsed_time = 0.0f32;
    let mut texture_cache: Vec<Texture2D> = Vec::new();
    let mut frame_count = 0u32;
    let texture_refresh_rate = 10;

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        elapsed_time += delta_time;

        handle_input(&rl, &mut camera_controller, &celestial_bodies, &mut warp_effect, &mut spaceship, &mut top_down);

        if rl.is_key_pressed(KeyboardKey::KEY_O) {
            show_orbits = !show_orbits;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_I) {
            show_info = !show_info;
        }

        camera_controller.update(&rl, delta_time);
        if top_down {
            camera_controller.camera.position = Vector3::new(0.0, 250.0, 0.01);
            camera_controller.camera.target = Vector3::zero();
        }
        update_celestial_bodies(&mut celestial_bodies, delta_time);
        spaceship.update(delta_time);
        warp_effect.update(delta_time);

        if let Some(safe_pos) = collision_system.check_and_resolve(
            camera_controller.camera.position,
            &celestial_bodies,
        ) {
            camera_controller.apply_collision(safe_pos, 2.0);
        }

        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);

        // Generar texturas una sola vez
        if texture_cache.is_empty() {
            for body in &celestial_bodies {
                let img = shader_manager.create_texture_for_body_named(
                    body.body_type,
                    body.color,
                    elapsed_time,
                    &body.name,
                );
                let tex = d.load_texture_from_image(&thread, &img).unwrap();
                texture_cache.push(tex);
            }
        }
        frame_count += 1;

        {
            let mut d3 = d.begin_mode3D(camera_controller.camera);

            skybox.draw(&mut d3, Vector3::zero());
            skybox.draw_stars(&mut d3, Vector3::zero());

            if show_orbits {
                for body in &celestial_bodies {
                    if !body.is_sun && body.parent.is_none() {
                        orbit_renderer.draw_orbit(
                            &mut d3,
                            Vector3::zero(),
                            body.orbital_radius,
                            Color::new(100, 100, 100, 100),
                        );
                    }
                }
            }

            let mut saturn_pos: Option<Vector3> = None;
            for (idx, body) in celestial_bodies.iter().enumerate() {
                if idx < texture_cache.len() {
                    let tex = &texture_cache[idx];
                    unsafe {
                        use raylib::consts::MaterialMapIndex;
                        let mat_ptr = sphere_model.materials_mut().as_mut_ptr();
                        (*mat_ptr).maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
                            .texture = **tex;
                    }
                    
                    d3.draw_model_ex(
                        &sphere_model,
                        body.position,
                        Vector3::new(0.0, 1.0, 0.0),
                        0.0,
                        Vector3::new(body.radius, body.radius, body.radius),
                        Color::WHITE,
                    );
                }
                
                if body.name == "Saturno" { saturn_pos = Some(body.position); }
                if body.is_sun {
                    d3.draw_sphere(body.position, body.radius * 1.08, Color::new(255, 220, 100, 8));
                }
            }
            if let Some(pos) = saturn_pos {
                let segments = 180;
                let tilt_angle = 15.0_f32.to_radians();
                
                let ring_bands = [
                    (5.2, 5.8, Color::new(200, 180, 140, 140)),
                    (5.8, 6.3, Color::new(190, 170, 130, 150)),
                    (6.3, 6.8, Color::new(180, 160, 120, 135)),
                    // Gap de Cassini (6.8 - 7.1)
                    (7.1, 7.5, Color::new(210, 190, 150, 145)),
                    (7.5, 8.0, Color::new(195, 175, 140, 155)),
                    (8.0, 8.5, Color::new(185, 165, 130, 140)),
                    (8.5, 9.0, Color::new(170, 150, 115, 120)),
                ];
                
                for (ring_inner, ring_outer, base_color) in &ring_bands {
                    let num_circles = 15;
                    for circle_idx in 0..num_circles {
                        let t = circle_idx as f32 / num_circles as f32;
                        let radius = ring_inner + (ring_outer - ring_inner) * t;
                        
                        let brightness = 1.0 + (t - 0.5) * 0.2;
                        let circle_color = Color::new(
                            (base_color.r as f32 * brightness).clamp(0.0, 255.0) as u8,
                            (base_color.g as f32 * brightness).clamp(0.0, 255.0) as u8,
                            (base_color.b as f32 * brightness).clamp(0.0, 255.0) as u8,
                            base_color.a,
                        );
                        
                        for i in 0..segments {
                            let a1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
                            let a2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
                            
                            let noise_val = ((a1 * 23.0).sin() * 0.5 + 0.5) * 0.1;
                            let varied_brightness = brightness + noise_val - 0.05;
                            let varied_color = Color::new(
                                (base_color.r as f32 * varied_brightness).clamp(0.0, 255.0) as u8,
                                (base_color.g as f32 * varied_brightness).clamp(0.0, 255.0) as u8,
                                (base_color.b as f32 * varied_brightness).clamp(0.0, 255.0) as u8,
                                base_color.a,
                            );
                            
                            let y_offset = radius * tilt_angle.sin();
                            let z_scale = tilt_angle.cos();
                            
                            let p1 = Vector3::new(
                                pos.x + radius * a1.cos(),
                                pos.y + y_offset * a1.sin(),
                                pos.z + radius * a1.sin() * z_scale
                            );
                            let p2 = Vector3::new(
                                pos.x + radius * a2.cos(),
                                pos.y + y_offset * a2.sin(),
                                pos.z + radius * a2.sin() * z_scale
                            );
                            
                            d3.draw_line_3D(p1, p2, varied_color);
                        }
                    }
                }
            }

            spaceship.draw(
                &mut d3,
                &ship_model,
                camera_controller.camera.position,
                camera_controller.camera.target,
            );

            if warp_effect.is_active() {
                warp_effect.draw(&mut d3);
            }
        }

        if show_info {
            draw_ui(&mut d, &camera_controller, &celestial_bodies);
        }

        d.draw_fps(10, 10);
    }
}

fn create_solar_system() -> Vec<CelestialBody> {
    let mut bodies = vec![
        CelestialBody::new_sun("Sol", 8.0, Color::new(255, 200, 50, 255)),
        
        CelestialBody::new_planet("Mercurio", 1.5, Color::GRAY, 15.0, 4.0, 2.0),
        
        {
            let mut venus = CelestialBody::new_planet("Venus", 2.0, Color::ORANGE, 22.0, 3.5, 1.8);
            venus.body_type = BodyType::GasGiant;
            venus
        },
        
        CelestialBody::new_planet("Tierra", 2.2, Color::BLUE, 30.0, 3.0, 1.5),
        
        CelestialBody::new_moon("Luna", 0.6, Color::LIGHTGRAY, 4.0, 8.0, 1.0, 3),
        
        {
            let mut marte = CelestialBody::new_planet("Marte", 1.8, Color::RED, 40.0, 2.5, 1.4);
            marte.body_type = BodyType::GasGiant;
            marte
        },
        
        CelestialBody::new_moon("Fobos", 0.3, Color::DARKGRAY, 3.5, 12.0, 2.0, 5),
        
        CelestialBody::new_moon("Deimos", 0.25, Color::GRAY, 5.0, 10.0, 1.5, 5),
        
        CelestialBody::new_planet("Jupiter", 5.0, Color::BROWN, 60.0, 1.3, 1.0),
        
        {
            let mut saturno = CelestialBody::new_planet("Saturno", 4.5, Color::BEIGE, 80.0, 1.0, 0.9);
            saturno.body_type = BodyType::GasGiant;
            saturno
        },
        
        CelestialBody::new_moon("Titan", 0.8, Color::ORANGE, 7.0, 6.0, 1.0, 9),
        
        CelestialBody::new_planet("Urano", 3.5, Color::SKYBLUE, 100.0, 0.7, 0.8),
        
        {
            let mut neptuno = CelestialBody::new_planet("Neptuno", 3.5, Color::DARKBLUE, 120.0, 0.5, 0.7);
            neptuno.body_type = BodyType::GasGiant;
            neptuno
        },
    ];
    
    bodies
}

fn load_textures_for_bodies(rl: &mut RaylibHandle, thread: &RaylibThread, bodies: &mut [CelestialBody]) {
    let map: [(&str, &str); 13] = [
        ("Sol", "assets/textures/sol.png"),
        ("Mercurio", "assets/textures/mercurio.png"),
        ("Venus", "assets/textures/venus.png"),
        ("Tierra", "assets/textures/tierra.png"),
        ("Luna", "assets/textures/luna.png"),
        ("Marte", "assets/textures/marte.png"),
        ("Fobos", "assets/textures/fobos.png"),
        ("Deimos", "assets/textures/deimos.png"),
        ("Jupiter", "assets/textures/jupiter.png"),
        ("Saturno", "assets/textures/saturno.png"),
        ("Urano", "assets/textures/urano.png"),
        ("Neptuno", "assets/textures/neptuno.png"),
        ("Titan", "assets/textures/titan.png"),
    ];

    for body in bodies.iter_mut() {
        if let Some((_, path)) = map.iter().find(|(n, _)| *n == body.name) {
            if let Ok(tex) = rl.load_texture(thread, path) {
                body.texture = Some(tex);
            }
        }
    }
}

fn update_celestial_bodies(bodies: &mut Vec<CelestialBody>, delta_time: f32) {
    let body_count = bodies.len();
    
    for i in 0..body_count {
        let parent_pos = if let Some(parent_idx) = bodies[i].parent {
            Some(bodies[parent_idx].position)
        } else {
            None
        };
        
        bodies[i].update(delta_time, parent_pos);
    }
}

fn handle_input(
    rl: &RaylibHandle,
    camera_controller: &mut CameraController,
    bodies: &[CelestialBody],
    warp_effect: &mut WarpEffect,
    spaceship: &mut Spaceship,
    top_down: &mut bool,
) {
    if rl.is_key_pressed(KeyboardKey::KEY_ZERO) {
        let target = Vector3::new(0.0, 20.0, 40.0);
        camera_controller.start_warp(target);
        warp_effect.start(camera_controller.camera.position, target);
    }
    
    if rl.is_key_pressed(KeyboardKey::KEY_ONE) && bodies.len() > 1 {
        let target = bodies[1].position + Vector3::new(0.0, 5.0, 15.0);
        camera_controller.start_warp(target);
        warp_effect.start(camera_controller.camera.position, target);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_TWO) && bodies.len() > 2 {
        let target = bodies[2].position + Vector3::new(0.0, 5.0, 15.0);
        camera_controller.start_warp(target);
        warp_effect.start(camera_controller.camera.position, target);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_THREE) && bodies.len() > 3 {
        let target = bodies[3].position + Vector3::new(0.0, 5.0, 15.0);
        camera_controller.start_warp(target);
        warp_effect.start(camera_controller.camera.position, target);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FOUR) && bodies.len() > 5 {
        let target = bodies[5].position + Vector3::new(0.0, 5.0, 15.0);
        camera_controller.start_warp(target);
        warp_effect.start(camera_controller.camera.position, target);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FIVE) && bodies.len() > 8 {
        let target = bodies[8].position + Vector3::new(0.0, 5.0, 15.0);
        camera_controller.start_warp(target);
        warp_effect.start(camera_controller.camera.position, target);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_SIX) && bodies.len() > 9 {
        let target = bodies[9].position + Vector3::new(0.0, 5.0, 15.0);
        camera_controller.start_warp(target);
        warp_effect.start(camera_controller.camera.position, target);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_SEVEN) && bodies.len() > 11 {
        let target = bodies[11].position + Vector3::new(0.0, 5.0, 15.0);
        camera_controller.start_warp(target);
        warp_effect.start(camera_controller.camera.position, target);
    }
    if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) && bodies.len() > 12 {
        let target = bodies[12].position + Vector3::new(0.0, 5.0, 15.0);
        camera_controller.start_warp(target);
        warp_effect.start(camera_controller.camera.position, target);
    }

    if rl.is_key_pressed(KeyboardKey::KEY_V) {
        spaceship.toggle_orbit_demo();
    }

    if rl.is_key_pressed(KeyboardKey::KEY_B) {
        *top_down = !*top_down;
    }

    if rl.is_key_pressed(KeyboardKey::KEY_R) {
        let home_pos = Vector3::new(0.0, 30.0, 50.0);
        camera_controller.camera.position = home_pos;
        camera_controller.camera.target = Vector3::zero();
    }
}

fn draw_ui(d: &mut RaylibDrawHandle, camera: &CameraController, _bodies: &[CelestialBody]) {
    let y_start = 40;
    let line_height = 20;

    d.draw_text("=== CONTROLES ===", 10, y_start, 20, Color::WHITE);
    d.draw_text("WASD: Mover", 10, y_start + line_height, 16, Color::LIGHTGRAY);
    d.draw_text("Flechas: Rotar camara", 10, y_start + line_height * 2, 16, Color::LIGHTGRAY);
    d.draw_text("Espacio/Shift: Arriba/Abajo", 10, y_start + line_height * 3, 16, Color::LIGHTGRAY);
    d.draw_text("0-8: Warp a planetas", 10, y_start + line_height * 4, 16, Color::LIGHTGRAY);
    d.draw_text("O: Toggle orbitas", 10, y_start + line_height * 5, 16, Color::LIGHTGRAY);
    d.draw_text("I: Toggle info", 10, y_start + line_height * 6, 16, Color::LIGHTGRAY);
    d.draw_text("V: Toggle orbita nave", 10, y_start + line_height * 7, 16, Color::LIGHTGRAY);
    d.draw_text("B: Vista cenital", 10, y_start + line_height * 8, 16, Color::LIGHTGRAY);
    d.draw_text("B R: Ir a inicio", 10, y_start + line_height * 9, 16, Color::LIGHTGRAY);

    // Información de posición
    d.draw_text(
        &format!(
            "Posicion: X:{:.1} Y:{:.1} Z:{:.1}",
            camera.camera.position.x,
            camera.camera.position.y,
            camera.camera.position.z
        ),
        10,
        y_start + line_height * 11,
        16,
        Color::YELLOW,
    );
}