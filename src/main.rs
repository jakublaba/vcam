use std::f64::consts::PI;

use nalgebra::Vector3;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::brep::Brep;
use crate::enums::Axis;

mod brep;
mod matrix_templates;
mod enums;
mod cube_generator;

const VW: u32 = 800;
const VH: u32 = 600;
const MOVE_STEP: f64 = 10.;
const ZOOM_STEP: f64 = 0.1;
const ROT_STEP: f64 = 5. * PI / 180.;
const FOV: f64 = 60. * PI / 180.;
const AR: f64 = (VW / VH) as f64;
const NEAR: f64 = 0.1;
const FAR: f64 = 100.;

fn main() {
    let objects: Vec<Brep> = cube_generator::generate_cubes();
    for o in &objects {
        println!("{:?}", o)
    }

    let sld_ctx = sdl2::init().unwrap();
    let video_subsystem = sld_ctx.video().unwrap();
    let window = video_subsystem
        .window("Virtual Camera", VW, VH)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();
    let mut event_pump = sld_ctx.event_pump().unwrap();

    let mut translation_state = Vector3::new(0., 0., 0.);
    let mut rotation_state = Vector3::new(0., 0., 0.);
    let mut scale_state = 0.;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    // forward
                    translation_state.z += MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    // backward
                    translation_state.z -= MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    // left
                    translation_state.x -= MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    // right
                    translation_state.x += MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    // up
                    translation_state.y += MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    // down
                    translation_state.y -= MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    // zoom in
                    scale_state += ZOOM_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    // zoom out
                    scale_state -= ZOOM_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    // clockwise x
                    rotation_state.x += ROT_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    // counterclockwise x
                    rotation_state.x -= ROT_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    // clockwise y
                    rotation_state.y += ROT_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    // counterclockwise y
                    rotation_state.y -= ROT_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    // clockwise z
                    rotation_state.z += ROT_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::U), .. } => {
                    // counterclockwise z
                    rotation_state.z -= ROT_STEP;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);

        for obj in &objects {
            let transformed = obj
                .translate(translation_state)
                .scale(scale_state)
                .rotate(rotation_state.x, Axis::X)
                .rotate(rotation_state.y, Axis::Y)
                .rotate(rotation_state.z, Axis::Z)
                .cast_2d(FOV, AR, NEAR, FAR);
            for line in transformed.edges {
                let a = Point::new(
                    transformed.vertices[line[0] as usize].x as i32,
                    transformed.vertices[line[0] as usize].y as i32,
                );
                let b = Point::new(
                    transformed.vertices[line[1] as usize].x as i32,
                    transformed.vertices[line[1] as usize].y as i32,
                );
                canvas.draw_line(a, b).unwrap();
            }
        }

        canvas.present();
    }
}
