use cgmath::{Matrix4, Point3};

use crate::scene::polygon::Polygon;

pub(crate) mod polygon;
pub(crate) mod vertex;

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

    pub fn sorted(&self, camera_position: Point3<f64>) -> Scene {
        let mut polygons_sorted = self.polygons.clone();
        polygons_sorted
            .sort_by(|a, b| {
                a.distance_to_camera(camera_position)
                    .total_cmp(&b.distance_to_camera(camera_position))
            });

        Scene::new(polygons_sorted)
    }
}
