use cgmath::{Deg, Matrix4, Point3, Transform, Vector3};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::brep::Brep;

mod brep;
mod cube_generator;

const VW: u32 = 1920;
const VH: u32 = 1080;
const MOVE_STEP: f64 = 10.;
const LOOK_STEP: f64 = 10.;
const ZOOM_STEP: f64 = 5.;
const TILT_STEP: f64 = 5.;
const AR: f64 = (VW / VH) as f64;
const NEAR: f64 = 0.1;
const FAR: f64 = 100.;

fn main() -> Result<(), String> {
    let mut objects: Vec<Brep> = cube_generator::generate_cubes();
    for o in &objects {
        println!("{:?}", o)
    }

    let sdl_ctx = sdl2::init()?;
    let video_subsystem = sdl_ctx.video()?;
    let window = video_subsystem
        .window("Virtual Camera", VW, VH)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();

    let mut position = Point3::new(0., 0., 0.);
    let mut direction = Vector3::new(0., 0., 1.);
    let mut fov = 45.;
    let mut up = Vector3::new(0., 1., 0.);

    let mut event_pump = sdl_ctx.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    // forward
                    position.z += MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    // backward
                    position.z -= MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    // left
                    position.x += MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    // right
                    position.x -= MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    // up
                    position.y -= MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    // down
                    position.y += MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    // zoom in
                    if fov < 90. { fov += ZOOM_STEP; }
                }
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    // zoom out
                    if fov > 30. { fov -= ZOOM_STEP; }
                }
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    // look up
                    direction = Matrix4::from_angle_x(Deg(LOOK_STEP)).transform_vector(direction);
                }
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    // look down
                    direction = Matrix4::from_angle_x(Deg(-LOOK_STEP)).transform_vector(direction);
                }
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    // look left
                    direction = Matrix4::from_angle_y(Deg(LOOK_STEP)).transform_vector(direction);
                }
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    // look right
                    direction = Matrix4::from_angle_y(Deg(-LOOK_STEP)).transform_vector(direction);
                }
                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    // tilt left
                    up = Matrix4::from_angle_z(Deg(-TILT_STEP)).transform_vector(up);
                }
                Event::KeyDown { keycode: Some(Keycode::U), .. } => {
                    // tilt right
                    up = Matrix4::from_angle_z(Deg(TILT_STEP)).transform_vector(up);
                }
                _ => {}
            }
        }

        let view = Matrix4::look_to_rh(position, direction, up);

        canvas.set_draw_color(Color::RED);
        for obj in &mut objects {
            let t = obj
                .transform(view)
                .project_2d(fov, AR, NEAR, FAR)
                .screen_coords(VW, VH);
            for line in &t.edges {
                let a = Point::new(
                    t.vertices[line.0].x as i32,
                    t.vertices[line.0].y as i32,
                );
                let b = Point::new(
                    t.vertices[line.1].x as i32,
                    t.vertices[line.1].y as i32,
                );

                canvas.draw_line(a, b)?;
            }
        }

        canvas.present();
    };

    Ok(())
}
