use nalgebra::{Matrix4, Point3, Vector3};

use crate::enums::Axis;
use crate::matrix_templates;

#[derive(Debug)]
pub struct Brep {
    pub vertices: Vec<Point3<f64>>,
    pub edges: Vec<[i32; 2]>,
}

impl Brep {
    pub fn new(vertices: Vec<Point3<f64>>, edges: Vec<[i32; 2]>) -> Brep {
        Brep { vertices, edges }
    }

    pub fn translate(&self, v: Vector3<f64>) -> Brep {
        let translation = matrix_templates::translation(v);
        let v = self.transform_points(translation);
        Brep::new(v, self.edges.clone())
    }

    pub fn scale(&self, scale: f64) -> Brep {
        let scaling = matrix_templates::scaling(scale);
        let v = self.transform_points(scaling);
        Brep::new(v, self.edges.clone())
    }

    pub fn rotate(&self, rot: f64, axis: Axis) -> Brep {
        let rotation = match axis {
            Axis::X => matrix_templates::x_rotation(rot),
            Axis::Y => matrix_templates::y_rotation(rot),
            Axis::Z => matrix_templates::z_rotation(rot),
        };
        let v = self.transform_points(rotation);
        Brep::new(v, self.edges.clone())
    }

    pub fn cast_2d(&self, fov: f64, ar: f64, near: f64, far: f64) -> Brep {
        let projection = matrix_templates::projection(fov, ar, near, far);
        let v = self.transform_points(projection);
        Brep::new(v, self.edges.clone())
    }

    fn transform_points(&self, transform_matrix: Matrix4<f64>) -> Vec<Point3<f64>> {
        self.vertices.iter()
            .map(|v| Point3::from_homogeneous(transform_matrix * v.to_homogeneous()).unwrap())
            .collect()
    }
}
