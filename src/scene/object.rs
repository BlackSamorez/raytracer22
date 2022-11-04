use std::rc::{Rc, Weak};

use crate::geometry::polygon::Polygon;
use crate::geometry::vector::Vector3D;
use crate::scene::material::Material;

pub struct Object {
    pub polygon: Polygon,
    pub material: Rc<Material>,
}

pub struct PseudoObject {
    pub material: Weak<Material>,
    pub first_point_idx: usize,
    pub second_point_idx: usize,
    pub third_point_idx: usize,
}

impl PseudoObject {
    pub fn build_object(&self, vertices: &Vec<Vector3D>, normals: &Vec<Vector3D>) -> Object {
        Object {
            material: self.material.upgrade().unwrap(),
            polygon: Polygon {
                first_point: vertices[self.first_point_idx].clone(),
                second_point: vertices[self.second_point_idx].clone(),
                third_point: vertices[self.third_point_idx].clone(),
                first_normal: normals[self.first_point_idx].clone(),
                second_normal: normals[self.second_point_idx].clone(),
                third_normal: normals[self.third_point_idx].clone(),
            },
        }
    }
}
