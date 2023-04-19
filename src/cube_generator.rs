use cgmath::{Point3, Vector3};

use crate::brep::Brep;

pub fn generate_cubes() -> Vec<Brep> {
    let cube_positions = [
        Vector3::new(0., 0., 0.),
        Vector3::new(200., 0., 0.),
        Vector3::new(0., 200., 0.),
        Vector3::new(200., 200., 0.),
    ];
    let mut cubes = Vec::new();
    for position in cube_positions {
        let t = cube_template()
            .scale(100.)
            .translate(position);
        cubes.push(t);
    }
    cubes
}

fn cube_template() -> Brep {
    Brep {
        vertices: vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(0.0, 1.0, 1.0),
        ],
        edges: vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ],
    }
}

