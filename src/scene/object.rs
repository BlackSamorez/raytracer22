use std::sync::Arc;

use crate::geometry::polygon::Polygon;
use crate::geometry::vector::Vector3D;
use crate::scene::material::Material;

pub struct Object {
    pub polygon: Polygon,
    pub material: Arc<Material>,
}

pub struct PseudoObject {
    pub material: Arc<Material>,
    pub first_vertex_idx: usize,
    pub second_vertex_idx: usize,
    pub third_vertex_idx: usize,
}

impl PseudoObject {
    pub fn build_object(&self, vertices: &[Vector3D], normals: &[Vector3D]) -> Object {
        Object {
            material: Arc::clone(&self.material),
            polygon: Polygon {
                first_point: vertices[self.first_vertex_idx].clone(),
                second_point: vertices[self.second_vertex_idx].clone(),
                third_point: vertices[self.third_vertex_idx].clone(),
                first_normal: normals[self.first_vertex_idx].clone(),
                second_normal: normals[self.second_vertex_idx].clone(),
                third_normal: normals[self.third_vertex_idx].clone(),
            },
        }
    }
}
