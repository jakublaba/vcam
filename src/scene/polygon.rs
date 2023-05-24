use cgmath::{EuclideanSpace, Point3};
use sdl2::pixels::Color;

use crate::scene::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Vertex>,
    pub color: Color,
}

impl Polygon {
    pub fn with_color(&self, color: Color) -> Self {
        Self {
            vertices: self.vertices.clone(),
            color,
        }
    }

    pub fn from_vertices(vertices: Vec<Vertex>, color: Color) -> Self {
        Self { vertices, color }
    }

    pub fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    pub fn transform(&self, transform_matrix: cgmath::Matrix4<f64>) -> Polygon {
        let v = self
            .vertices
            .iter()
            .map(|v| v.transform(transform_matrix))
            .collect();
        Polygon::from_vertices(v, self.color)
    }

    pub fn screen_coords(&self, vw: u32, vh: u32) -> Option<Polygon> {
        let v: Vec<Option<Vertex>> = self
            .vertices
            .iter()
            .map(|v| v.screen_coords(vw, vh))
            .collect();

        if v.iter().any(|v| v.is_none()) {
            return None;
        }

        let v: Vec<Vertex> = v.into_iter().map(|v| v.unwrap()).collect();

        Some(Polygon::from_vertices(v, self.color))
    }

    pub(crate) fn centroid(&self) -> Point3<f64> {
        Point3::centroid(
            &self
                .vertices
                .iter()
                .map(|v| v.position())
                .collect::<Vec<_>>(),
        )
    }
}
