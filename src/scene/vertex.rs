use cgmath::{Matrix4, Point3, Transform};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: Point3<f64>,
}

impl Vertex {
    pub fn from_point3(position: Point3<f64>) -> Self {
        Self { position }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            position: Point3::new(x, y, z),
        }
    }

    pub fn x(&self) -> f64 {
        self.position.x.clone()
    }

    pub fn y(&self) -> f64 {
        self.position.y.clone()
    }

    pub fn z(&self) -> f64 {
        self.position.z.clone()
    }

    pub fn is_visible(&self, pos: Point3<f64>, near: f64, far: f64) -> bool {
        let clip_range = (pos.z + near)..=(pos.z + far);
        let is_vis = clip_range.contains(&self.position.z);
        is_vis
    }

    pub fn transform(&self, transform_matrix: Matrix4<f64>) -> Vertex {
        Vertex::from_point3(transform_matrix.transform_point(self.position))
    }

    pub fn screen_coords(&self, vw: u32, vh: u32) -> Vertex {
        let x = (self.x() + 1.) * 0.5 * vw as f64;
        let y = (self.y() + 1.) * 0.5 * vh as f64;
        let z = 0.;

        Vertex::new(x, y, z)
    }
}
