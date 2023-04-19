use nalgebra::{Point, point, Point3, vector};

use crate::brep::Brep;

pub fn generate_cubes() -> Vec<Brep> {
    let cube_positions = [
        vector![0., 0., 0.],
        vector![50., 0., 0.],
        vector![0., 50., 0.],
        vector![50., 50., 0.]
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
            point![0.0, 0.0, 0.0],
            point![50.0, 0.0, 0.0],
            point![50.0, 50.0, 0.0],
            point![0.0, 50.0, 0.0],
            point![0.0, 0.0, 50.0],
            point![50.0, 0.0, 50.0],
            point![50.0, 50.0, 50.0],
            point![0.0, 50.0, 50.0],
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

