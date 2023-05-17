use std::vec;
use crate::scene::polygon::Polygon;
use crate::scene::vertex::Vertex;

pub fn generate_cubes() -> Vec<Polygon> {
    let polygons = vec![Polygon::from_vertices(vec![
        Vertex::new(10., 10., 10.),
        Vertex::new(-10., -10., -10.),
        Vertex::new(10., -10., 10.),
    ])];

    polygons
}
