use cgmath::{Decomposed, Matrix4, Point3, Quaternion, Transform, Vector3};

use crate::{AR, FOV, VH, VW};

#[derive(Debug)]
pub struct Brep {
    pub vertices: Vec<Point3<f64>>,
    pub edges: Vec<(usize, usize)>,
}

impl Brep {
    pub fn new(vertices: Vec<Point3<f64>>, edges: Vec<(usize, usize)>) -> Brep {
        Brep { vertices, edges }
    }

    pub fn translate(&self, v: Vector3<f64>) -> Brep {
        let transl = Matrix4::from_translation(v);
        let v = self.vertices.iter()
            .map(|v| transl.transform_point(*v))
            .collect();
        Brep::new(v, self.edges.clone())
    }

    pub fn scale(&self, s: f64) -> Brep {
        let scale = Matrix4::from_scale(s);
        let v = self.vertices.iter()
            .map(|v| scale.transform_point(*v))
            .collect();
        Brep::new(v, self.edges.clone())
    }

    pub fn transform(&mut self, transform_matrix: Matrix4<f64>) {
        println!("Transformation");
        for v in &mut self.vertices {
            *v = transform_matrix.transform_point(*v);
        }
    }

    pub fn cast_2d(&mut self, fov: f64, ar: f64, near: f64, far: f64) {
        println!("Projection");
        let projection = cgmath::perspective(cgmath::Deg(fov), ar, near, far);
        for v in &mut self.vertices {
            *v = projection.transform_point(*v);
        }
    }

    pub fn screen_coords(&mut self, vw: u32, vh: u32) {
        println!("Coord transformation");
        for v in &mut self.vertices {
            v.x = (v.x + 1.) * 0.5 * vw as f64;
            v.x = (v.y + 1.) * 0.5 * vh as f64;
        }
    }
}
