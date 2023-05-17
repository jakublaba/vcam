use cgmath::Matrix4;

use crate::polygon::Polygon;

#[derive(Debug)]
pub struct Scene {
    polygons: Vec<Polygon>,
}

impl Scene {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        Self { polygons }
    }

    pub fn polygons(&self) -> Vec<Polygon> {
        self.polygons.clone()
    }

    pub fn transform(&self, transform_matrix: Matrix4<f64>) -> Scene {
        let polygons_transformed = self
            .polygons
            .iter()
            .map(|p| p.transform(transform_matrix))
            .collect();
        Scene::new(polygons_transformed)
    }

    pub fn screen_coords(&self, vw: u32, vh: u32) -> Scene {
        let projected_polygons = self
            .polygons
            .iter()
            .map(|p| p.screen_coords(vw, vh))
            .collect();

        Scene::new(projected_polygons)
    }
}
