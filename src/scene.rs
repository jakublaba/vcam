use cgmath::{Matrix4, Point3};

use crate::polygon::Polygon;

#[derive(Debug)]
pub struct Scene {
    polygons: Vec<Polygon>,
}

impl Scene {
    pub fn new(polygons: Vec<Polygon>) -> Scene {
        Scene { polygons }
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

    pub fn projected_to_view(&self, vw: u32, vh: u32) -> Scene {
        let projected_polygons = self
            .polygons
            .iter()
            .map(|p| p.project_to_view(vw, vh))
            .collect();

        Scene::new(projected_polygons)
    }
    // pub fn projected_to_view(&self, vw: u32, vh: u32) -> Scene {
    //     let projected_polygons = self
    //         .polygons
    //         .iter()
    //         .map(|p| {
    //             let vertices = p
    //                 .vertices()
    //                 .iter()
    //                 .map(|v| {
    //                     let x = (v.position().x + 1.) * 0.5 * vw as f64;
    //                     let y = (v.position().y + 1.) * 0.5 * vh as f64;
    //                     let z = 0.;
    //                     Vertex::from_point3(Point3::new(x, y, z))
    //                 })
    //                 .collect();
    //             Polygon::from_vertices(vertices)
    //         })
    //         .collect();
    //
    //     Scene::new(projected_polygons)
    // }
    // pub fn screen_coords(&self, vw: u32, vh: u32) -> Scene {
    //     let vertices_on_screen = self
    //         .polygons
    //         .iter()
    //         .flat_map(|p| p.vertices())
    //         .map(|v| {
    //             Polygon::from_vertices(Vertex::new(Point3::new(
    //                 (v.position().x + 1.) * 0.5 * vw as f64,
    //                 (v.position().y + 1.) * 0.5 * vh as f64,
    //                 0.,
    //             )))
    //         })
    //         .collect();
    //
    //     Scene::new(vertices_on_screen)
    // }
}
