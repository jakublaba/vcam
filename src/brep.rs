use cgmath::{Decomposed, Matrix4, Point3, Quaternion, Transform, Vector3};

use crate::{AR, FOV, matrix_templates, VH, VW};
use crate::enums::Axis;
use crate::matrix_templates::translation;

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
        let transl = cgmath::Decomposed {
            scale: 1.,
            rot: Quaternion::new(1., 0., 0., 0.),
            disp: v,
        };
        let v = self.vertices.iter()
            .map(|v| transl.transform_point(*v))
            .collect();
        Brep::new(v, self.edges.clone())
    }

    pub fn transform(&self, transform_matrix: Matrix4<f64>) -> Brep {
        println!("Transformation");
        let v = self.vertices.iter()
            .map(|v| transform_matrix.transform_point(*v))
            .collect();
        Brep::new(v, self.edges.clone())
    }

    pub fn cast_2d(&self, fov: f64, ar: f64, near: f64, far: f64) -> Brep {
        println!("Projection");
        let projection = cgmath::perspective(cgmath::Deg(fov), ar, near, far);
        let v = self.vertices.iter()
            .map(|v| projection.transform_point(*v))
            .collect();
        Brep::new(v, self.edges.clone())
    }

    pub fn to_screen_coords(&self, vw: u32, vh: u32) -> Brep {
        println!("Coord transformation");
        let v = self.vertices.iter()
            .map(|v| {
                let x_screen = (v.x + 1.) * 0.5 * vw as f64;
                let y_screen = (v.y + 1.) * 0.5 * vh as f64;
                Point3::new(x_screen, y_screen, 0.)
            })
            .collect();
        Brep::new(v, self.edges.clone())
    }
}
