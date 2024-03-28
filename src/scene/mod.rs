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

    pub fn transform(&self, transform_matrix: Matrix4<f64>) -> Scene {
        let polygons_transformed = self
            .polygons
            .iter()
            .map(|p| p.transform(transform_matrix))
            .collect();
        Scene::new(polygons_transformed)
    }

    pub fn screen_coords(&self, vw: u32, vh: u32) -> Scene {
        let projected_polygons: Vec<Option<Polygon>> = self
            .polygons
            .iter()
            .map(|p| p.screen_coords(vw, vh))
            .collect();

        let projected_polygons_visible = projected_polygons
            .iter()
            .filter(|p| p.is_some())
            .map(|p| p.clone().unwrap())
            .collect();

        Scene::new(projected_polygons_visible)
    }

    // TODO: Sorting
    pub fn sorted(&self, camera_position: Point3<f64>) -> Scene {
        let mut polygons_sorted = self.polygons.clone();
        polygons_sorted.sort_by(|a, b| {
            let a_centr = a.centroid();
            let b_centr = b.centroid();
            let a_dist = a_centr.distance2(camera_position);
            let b_dist = b_centr.distance2(camera_position);
            b_dist.total_cmp(&a_dist)
        });

        Scene::new(polygons_sorted)
    }
}
