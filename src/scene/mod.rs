pub(crate) mod polygon;
pub(crate) mod vertex;

use cgmath::{Matrix4, Point3};

use crate::scene::polygon::Polygon;

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

    pub fn clip(&self, pos: Point3<f64>, near: f64, far: f64) -> Scene {
        let polygons_clipped = self
            .polygons
            .iter()
            .filter(|p| p.is_visible(pos, near, far))
            .map(|p| p.clone())
            .collect();
        Scene::new(polygons_clipped)
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
