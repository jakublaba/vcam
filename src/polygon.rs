use crate::vertex;
use cgmath::Point3;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<vertex::Vertex>,
    edges: Vec<Edge>,
}

impl Polygon {
    pub fn from_vertices(vertices: Vec<vertex::Vertex>) -> Self {
        let mut edges = Vec::new();
        for i in 0..vertices.len() {
            edges.push(Edge::new(vertices[i], vertices[(i + 1) % vertices.len()]));
        }
        Self { vertices, edges }
    }

    pub fn vertices(&self) -> Vec<vertex::Vertex> {
        self.vertices.clone()
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.edges.clone()
    }

    pub fn is_visible(&self, pos: Point3<f64>, near: f64, far: f64) -> bool {
        for e in &self.edges {
            if !e.is_visible(pos, near, far) {
                return false;
            }
        }
        true
    }

    pub fn transform(&self, transform_matrix: cgmath::Matrix4<f64>) -> Polygon {
        let v = self
            .vertices
            .iter()
            .map(|v| v.transform(transform_matrix))
            .collect();
        Polygon::from_vertices(v)
    }

    pub fn screen_coords(&self, vw: u32, vh: u32) -> Polygon {
        let v = self
            .vertices
            .iter()
            .map(|v| v.screen_coords(vw, vh))
            .collect();

        Polygon::from_vertices(v)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    // TODO - improve implementation to avoid storing duplicate vertices
    v1: vertex::Vertex,
    v2: vertex::Vertex,
}

impl Edge {
    pub fn new(v1: vertex::Vertex, v2: vertex::Vertex) -> Edge {
        Edge { v1, v2 }
    }

    pub fn vertices(&self) -> (vertex::Vertex, vertex::Vertex) {
        (self.v1, self.v2)
    }

    pub fn is_visible(&self, pos: Point3<f64>, near: f64, far: f64) -> bool {
        self.v1.is_visible(pos, near, far) && self.v2.is_visible(pos, near, far)
    }
}
