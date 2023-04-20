use cgmath::{Deg, Matrix4, Point3, Transform, Vector3};

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

    pub fn transform(&self, transform_matrix: Matrix4<f64>) -> Brep {
        let v = self.vertices.iter()
            .map(|v| transform_matrix.transform_point(*v))
            .collect();
        Brep::new(v, self.edges.clone())
    }

    pub fn project_2d(&self, fov: f64, ar: f64, near: f64, far: f64) -> Brep {
        let projection = cgmath::perspective(Deg(fov), ar, near, far);
        let v = self.vertices.iter()
            .map(|v| projection.transform_point(*v))
            .collect();
        Brep::new(v, self.edges.clone())
    }

    pub fn screen_coords(&self, vw: u32, vh: u32) -> Brep {
        let v = self.vertices.iter()
            .map(|v| Point3::new(
                (v.x + 1.) * 0.5 * vw as f64,
                (v.y + 1.) * 0.5 * vh as f64,
                0.,
            ))
            .collect();
        Brep::new(v, self.edges.clone())
    }
}
