use cgmath::{MetricSpace, Point3};

use crate::scene::polygon::Coord::{X, Y, Z};
use crate::scene::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

enum Coord {
    X,
    Y,
    Z,
}

// impl Eq for Polygon {}
//
// impl PartialEq<Self> for Polygon {
//     fn eq(&self, other: &Self) -> bool {
//         (&self.edges, &self.vertices) == (&other.edges, &other.vertices)
//     }
// }
//
// impl PartialOrd<Self> for Polygon {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl Ord for Polygon {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.centroid()
//         todo!()
//     }
// }

impl Polygon {
    pub fn from_vertices(vertices: Vec<Vertex>) -> Self {
        let mut edges = Vec::new();
        for i in 0..vertices.len() {
            edges.push(Edge::new(vertices[i], vertices[(i + 1) % vertices.len()]));
        }
        Self { vertices, edges }
    }

    pub fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
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

    pub fn distance_to_camera(&self, camera_pos: Point3<f64>) -> f64 {
        camera_pos.distance(self.centroid())
    }

    fn min_coord(&self, coord: Coord) -> f64 {
        let mut min = f64::MAX;

        for e in &self.edges {
            min = min.min(match coord {
                X => e.v1.x().min(e.v2.x()),
                Y => e.v1.y().min(e.v2.y()),
                Z => e.v1.z().min(e.v2.z()),
            });
        }

        min
    }

    fn max_coord(&self, coord: Coord) -> f64 {
        let mut max = f64::MIN;

        for e in &self.edges {
            max = max.max(match coord {
                X => e.v1.x().max(e.v2.x()),
                Y => e.v1.y().max(e.v2.y()),
                Z => e.v1.z().max(e.v2.z()),
            })
        }

        max
    }

    fn centroid(&self) -> Point3<f64> {
        Point3::new(
            (self.max_coord(X) + self.min_coord(X)) / 2.,
            (self.max_coord(Y) + self.min_coord(Y)) / 2.,
            (self.max_coord(Z) + self.min_coord(Z)) / 2.,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    // TODO - improve implementation to avoid storing duplicate vertices
    v1: Vertex,
    v2: Vertex,
}

impl Edge {
    pub fn new(v1: Vertex, v2: Vertex) -> Edge {
        Edge { v1, v2 }
    }

    pub fn is_visible(&self, pos: Point3<f64>, near: f64, far: f64) -> bool {
        self.v1.is_visible(pos, near, far) && self.v2.is_visible(pos, near, far)
    }
}
