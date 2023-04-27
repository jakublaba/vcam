use cgmath::{Deg, InnerSpace, Matrix4, perspective, Point3, Quaternion, Rotation, Rotation3, Vector3};
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
const NEAR: f64 = 1.;
const FAR: f64 = 100.;
const ANIMATION_INTERVAL: i32 = 5;

fn main() -> Result<(), String> {
    let mut objects: Vec<Brep> = cube_generator::generate_cubes();

    let sdl_ctx = sdl2::init()?;
    let video_subsystem = sdl_ctx.video()?;
    let window = video_subsystem
        .window("Virtual Camera", VW, VH)
        .position_centered()
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
    let mut forward = Vector3::new(0., 0., 1.);
    let mut fov = 45.;
    let mut up = Vector3::new(0., 1., 0.);

    let mut tick = 0;
    let mut event_pump = sdl_ctx.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    // forward
                    position += forward.normalize() * MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    // backward
                    position -= forward.normalize() * MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    // left
                    position -= forward.cross(up).normalize() * MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    // right
                    position += forward.cross(up).normalize() * MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    // up
                    position += up.normalize() * MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    // down
                    position -= up.normalize() * MOVE_STEP;
                }
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    // zoom in
                    if fov > 30. { fov -= ZOOM_STEP; }
                }
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    // zoom out
                    if fov < 90. { fov += ZOOM_STEP; }
                }
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    // look up
                    forward = Quaternion::from_axis_angle(forward.cross(up), Deg(LOOK_STEP)).rotate_vector(forward);
                }
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    // look down
                    forward = Quaternion::from_axis_angle(forward.cross(up), Deg(-LOOK_STEP)).rotate_vector(forward);
                }
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    // look left
                    forward = Quaternion::from_axis_angle(up, Deg(LOOK_STEP)).rotate_vector(forward);
                }
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    // look right
                    forward = Quaternion::from_axis_angle(up, Deg(-LOOK_STEP)).rotate_vector(forward);
                }
                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    // tilt left
                    up = Quaternion::from_axis_angle(forward, Deg(TILT_STEP)).rotate_vector(up);
                }
                Event::KeyDown { keycode: Some(Keycode::U), .. } => {
                    // tilt right
                    up = Quaternion::from_axis_angle(forward, Deg(-TILT_STEP)).rotate_vector(up);
                }
                _ => {}
            }
        }

        if tick == ANIMATION_INTERVAL {
            // some attempt at animating rotation
            objects = objects.iter()
                .map(|o| {
                    let rot_x = Matrix4::from_angle_x(Deg(3.));
                    let rot_y = Matrix4::from_angle_y(Deg(3.));
                    let rot_z = Matrix4::from_angle_z(Deg(3.));
                    o.transform(rot_z * rot_y * rot_x)
                })
                .collect();
        }

        let view = Matrix4::look_to_lh(position, forward, up);
        let projection = perspective(Deg(fov), AR, NEAR, FAR);

        canvas.set_draw_color(Color::BLACK);
        for obj in &objects {
            let t = obj
                .transform(view)
                .transform(projection)
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

        if tick < ANIMATION_INTERVAL {
            tick += 1;
        } else {
            tick = 0;
        }
    };

    Ok(())
}
