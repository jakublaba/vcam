use std::vec;

use crate::{polygon::Polygon, vertex::Vertex};

pub fn generate_cubes() -> Vec<Polygon> {
    let polygons = vec![Polygon::from_vertices(vec![
        Vertex::new(100., 100., 100.),
        Vertex::new(-100., -100., -100.),
        Vertex::new(100., -100., 100.),
    ])];

    polygons
}
