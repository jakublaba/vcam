use std::vec;

use crate::{polygon::Polygon, vertex::Vertex};

pub fn generate_cubes() -> Vec<Polygon> {
    let polygons = vec![Polygon::from_vertices(vec![
        Vertex::new(10., 10., 10.),
        Vertex::new(-10., -10., -10.),
        Vertex::new(10., -10., 10.),
    ])];

    polygons
}
