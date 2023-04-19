use cgmath::{Point3, Vector3};

use crate::brep::Brep;

pub fn generate_cubes() -> Vec<Brep> {
    let cube_positions = [
        Vector3::new(0., 0., 0.),
        Vector3::new(60., 0., 0.),
        Vector3::new(0., 60., 0.),
        Vector3::new(60., 60., 0.),
    ];
    let mut cubes = Vec::new();
    let template = cube_template();
    for position in cube_positions {
        let t = template.translate(position);
        cubes.push(t);
    }
    cubes
}

fn cube_template() -> Brep {
    Brep {
        vertices: vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(50.0, 0.0, 0.0),
            Point3::new(50.0, 50.0, 0.0),
            Point3::new(0.0, 50.0, 0.0),
            Point3::new(0.0, 0.0, 50.0),
            Point3::new(50.0, 0.0, 50.0),
            Point3::new(50.0, 50.0, 50.0),
            Point3::new(0.0, 50.0, 50.0),
        ],
        edges: vec![
            [0, 1],
            [1, 2],
            [2, 3],
            [3, 0],
            [4, 5],
            [5, 6],
            [6, 7],
            [7, 4],
            [0, 4],
            [1, 5],
            [2, 6],
            [3, 7],
        ],
    }
}

