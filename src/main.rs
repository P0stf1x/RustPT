#![deny(clippy::all)]
#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use minifb::{Key, Window, WindowOptions};

use glam::{Vec2, Vec3};

pub const WIDTH:  usize = 512;
pub const HEIGHT: usize = 288;

pub const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

mod importer;
use importer::Importer;

mod helper;
use crate::texture::Texture;
use crate::helper::*;
use crate::object::Object;
use crate::screen::ScreenBuffers;
use crate::polygon::{ Triangle, Vertex };
use crate::camera::Camera;
use crate::scene::Scene;

fn main() {
    let options = WindowOptions {
        borderless: false,
        title: true,
        resize: true,
        scale: minifb::Scale::X2,
        scale_mode: minifb::ScaleMode::AspectRatioStretch,
        topmost: true,
        transparency: false, // crash on macos
        none: false, //?
    };
    let mut window = Window::new(
        "RT",
        WIDTH,
        HEIGHT,
        options,
    ).unwrap_or_else( |e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    // window.set_target_fps(60);

    let screen = ScreenBuffers::new(WIDTH, HEIGHT);

    let tri = Triangle{
        vertices: [
            Vertex{pos: Vec3::new(0.0, 1.0, 0.0),  uv: Vec2::new(0.0, 1.0)},
            Vertex{pos: Vec3::new(-1.0, 0.0, 0.0), uv: Vec2::new(-1.0, 0.0)},
            Vertex{pos: Vec3::new(0.0, 0.0, 0.0),  uv: Vec2::new(0.0, 0.0)}
        ]
    };
    let reference_tri = Triangle{vertices: [Vertex{pos: Vec3::new(0.0, 100.0, 0.0), uv: Vec2::new(0.0, 100.0)}, Vertex{pos: Vec3::new(0.0, 0.0, 0.0), uv: Vec2::new(0.0, 0.0)}, Vertex{pos: Vec3::new(100.0, 0.0, 0.0), uv: Vec2::new(100.0, 0.0)}]};

    let mut ref_obj = Object::new(vec![reference_tri]);
    ref_obj.origin = Vec3::new(0., 0., -100.);
    
    let obj = Object::new(vec![tri]);

    let cube = Importer::obj("test/cube.obj");

    let mut teapot = Importer::obj("test/teapot_6320tri.obj");
    teapot.origin = Vec3::new(10., 0., 0.);
    println!("Total BVH leaves: {}", teapot.debug_get_bvh_end(teapot.bvh.clone()).len());
    teapot.debug_count_repeated_triangles();
    // let tris22 = cube.triangles.iter().collect();
    // BVH::generate_bottom(&tris22);

    let mut scene = Scene {
        screen: screen,
        objects: vec![
            obj,
            ref_obj,
            cube,
            teapot
        ],
        camera: {Camera::new(Vec3::new(0.0, 1.0, 2.0), Vec3::new(-80., 0., 0.))},
    };

    let mut frames_rendered = 0;
    let mut now = std::time::Instant::now();
    let mut show_depth_buffer = false;
    let temp_cam = Camera::new(Vec3::new(0.0, 0.0, 2.0), Vec3::ZERO);
    let temp_tex = Texture::new(WIDTH, HEIGHT);
    while window.is_open() && !window.is_key_down(Key::Escape) {

        scene.screen.clear();

        scene.camera.render(&scene.objects, &mut scene.screen);

        // Controls
            let speed_multiplier;
            // TODO: move outside of main
            // Camera controls
            if window.is_key_down(Key::LeftShift)  { speed_multiplier = 3. } else { speed_multiplier = 1. }
            if window.is_key_down(Key::W)          { scene.camera.translate(scene.camera.front().with_y(0.).normalize()*0.02 * speed_multiplier) }
            if window.is_key_down(Key::S)          { scene.camera.translate(scene.camera.back() .with_y(0.).normalize()*0.02 * speed_multiplier) }
            if window.is_key_down(Key::D)          { scene.camera.translate(scene.camera.right().with_y(0.).normalize()*0.02 * speed_multiplier) }
            if window.is_key_down(Key::A)          { scene.camera.translate(scene.camera.left() .with_y(0.).normalize()*0.02 * speed_multiplier) }
            if window.is_key_down(Key::Space)      { scene.camera.translate(Vec3::new( 0.00,  0.02,  0.00) * speed_multiplier) }
            if window.is_key_down(Key::LeftCtrl)   { scene.camera.translate(Vec3::new( 0.00, -0.02,  0.00) * speed_multiplier) }

            if window.is_key_down(Key::Up)         { scene.camera.rotate(Vec3::new( 0.0,  0.2,  0.0) * speed_multiplier) }
            if window.is_key_down(Key::Down)       { scene.camera.rotate(Vec3::new( 0.0, -0.2,  0.0) * speed_multiplier) }
            if window.is_key_down(Key::Left)       { scene.camera.rotate(Vec3::new( 0.2,  0.0,  0.0) * speed_multiplier) }
            if window.is_key_down(Key::Right)      { scene.camera.rotate(Vec3::new(-0.2,  0.0,  0.0) * speed_multiplier) }
            if window.is_key_down(Key::RightAlt)   { scene.camera.rotate(Vec3::new( 0.0,  0.0,  0.2) * speed_multiplier) }
            if window.is_key_down(Key::RightShift) { scene.camera.rotate(Vec3::new( 0.0,  0.0, -0.2) * speed_multiplier) }

            // Cube controls
            if window.is_key_down(Key::I)          { scene.objects[0].origin += scene.camera.front().with_y(0.).normalize()*0.02 * speed_multiplier }
            if window.is_key_down(Key::K)          { scene.objects[0].origin += scene.camera.back() .with_y(0.).normalize()*0.02 * speed_multiplier }
            if window.is_key_down(Key::L)          { scene.objects[0].origin += scene.camera.right().with_y(0.).normalize()*0.02 * speed_multiplier }
            if window.is_key_down(Key::J)          { scene.objects[0].origin += scene.camera.left() .with_y(0.).normalize()*0.02 * speed_multiplier }
            if window.is_key_down(Key::O)          { scene.objects[0].origin += Vec3::new( 0.00,  0.02,  0.00) * speed_multiplier }
            if window.is_key_down(Key::U)          { scene.objects[0].origin += Vec3::new( 0.00, -0.02,  0.00) * speed_multiplier }
            
            // Misc controls
            if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) { scene.camera.orthographic = !scene.camera.orthographic }
            if window.is_key_pressed(Key::B, minifb::KeyRepeat::No) { show_depth_buffer = !show_depth_buffer }
            if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) { scene.render_to_texture(&temp_cam, &temp_tex); temp_tex.save_to_file(); }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        if !show_depth_buffer {
            window
                .update_with_buffer(&scene.screen.get_rendered(), WIDTH, HEIGHT)
                .unwrap();
        } else {
            window
                .update_with_buffer(&scene.screen.get_depth(), WIDTH, HEIGHT)
                .unwrap();
        }

        frames_rendered += 1;
        if now.elapsed().as_millis() >= 1000 {
            let elapsed = now.elapsed();
            println!("{} fps,\t{} millis per frame", frames_rendered as f64/elapsed.as_secs_f64(), elapsed.as_millis() as f64/frames_rendered as f64);
            frames_rendered = 0;
            now = std::time::Instant::now();
        }
    };
}
