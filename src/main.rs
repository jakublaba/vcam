use cgmath::{
    perspective, Deg, InnerSpace, Matrix4, Point3, Quaternion, Rotation, Rotation3, Vector3,
};
use log::Level;
use polygon::Polygon;
use scene::Scene;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

mod cube_generator;
mod polygon;
mod scene;
mod vertex;

const VW: u32 = 800;
const VH: u32 = 600;
const MOVE_STEP: f64 = 10.;
const LOOK_STEP: f64 = 2.5;
const ZOOM_STEP: f64 = 5.;
const TILT_STEP: f64 = 5.;
const AR: f64 = (VW / VH) as f64;
const NEAR: f64 = 30.;
const FAR: f64 = 300.;
const FOV_MIN: f64 = 30.;
const FOV_MAX: f64 = 90.;
const FOV_DEFAULT: f64 = (FOV_MIN + FOV_MAX) / 2.;

fn main() -> Result<(), String> {
    simple_logger::init_with_level(Level::Debug)
        .map_err(|e| e.to_string())?;
    let objects: Vec<Polygon> = cube_generator::generate_cubes();

    let scene = Scene::new(objects);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
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
    let mut fov = FOV_DEFAULT;
    let mut up = Vector3::new(0., 1., 0.);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    // forward
                    position += forward.normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    // backward
                    position -= forward.normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    // left
                    position -= forward.cross(up).normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    // right
                    position += forward.cross(up).normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    // up
                    position += up.normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    // down
                    position -= up.normalize() * MOVE_STEP;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    // zoom in
                    if fov > FOV_MIN {
                        fov -= ZOOM_STEP;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => {
                    // zoom out
                    if fov < FOV_MAX {
                        fov += ZOOM_STEP;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    // reset zoom
                    fov = FOV_DEFAULT
                }
                Event::KeyDown {
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    // look up
                    forward = Quaternion::from_axis_angle(forward.cross(up), Deg(LOOK_STEP))
                        .rotate_vector(forward);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => {
                    // look down
                    forward = Quaternion::from_axis_angle(forward.cross(up), Deg(-LOOK_STEP))
                        .rotate_vector(forward);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => {
                    // look left
                    forward =
                        Quaternion::from_axis_angle(up, Deg(LOOK_STEP)).rotate_vector(forward);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    // look right
                    forward =
                        Quaternion::from_axis_angle(up, Deg(-LOOK_STEP)).rotate_vector(forward);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    // tilt left
                    up = Quaternion::from_axis_angle(forward, Deg(TILT_STEP)).rotate_vector(up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::U),
                    ..
                } => {
                    // tilt right
                    up = Quaternion::from_axis_angle(forward, Deg(-TILT_STEP)).rotate_vector(up);
                }
                _ => {}
            }
        }

        let view = Matrix4::look_to_lh(position, forward, up);
        let projection = perspective(Deg(fov), AR, NEAR, FAR);

        let projected_scene = scene
            .clip(position, NEAR, FAR)
            .transform(view)
            .transform(projection)
            .screen_coords(VW, VH);

        canvas.set_draw_color(Color::BLACK);
        for poly in &projected_scene.polygons() {
            poly.edges().iter().for_each(|edge| {
                let (a_vertex, b_vertex) = edge.vertices();

                log::debug!("a_vertex: x{} y{}", a_vertex.x(), a_vertex.y());
                log::debug!("b_vertex: x{} y{}", b_vertex.x(), b_vertex.y());

                let a = Point::new(a_vertex.x() as i32, a_vertex.y() as i32);
                let b = Point::new(b_vertex.x() as i32, b_vertex.y() as i32);

                log::debug!("a: x{} y{}", a.x(), a.y());
                log::debug!("b: x{} y{}", b.x(), b.y());

                canvas.draw_line(a, b).unwrap();
            });
        }

        canvas.present();
    }

    Ok(())
}
