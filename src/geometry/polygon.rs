use std::sync::Arc;

use super::vector::Vector3D;

#[derive(PartialEq)]
pub struct Polygon {
    pub first_point: Arc<Vector3D>,
    pub second_point: Arc<Vector3D>,
    pub third_point: Arc<Vector3D>,
    pub first_normal: Arc<Vector3D>,
    pub second_normal: Arc<Vector3D>,
    pub third_normal: Arc<Vector3D>,
}

impl Polygon {
    pub fn area(&self) -> f64 {
        let a = self.second_point.as_ref();
        (self.second_point.as_ref() - self.first_point.as_ref())
            .cross(&(self.third_point.as_ref() - self.first_point.as_ref()))
            .len()
    }

    pub fn weighted_normal(&self, x: &Vector3D) -> Vector3D {
        let area_abx = (self.second_point.as_ref() - self.first_point.as_ref())
            .cross(&(self.second_point.as_ref() - x))
            .len();
        let area_bcx = (self.third_point.as_ref() - self.second_point.as_ref())
            .cross(&(self.third_point.as_ref() - x))
            .len();
        let area_cax = (self.third_point.as_ref() - self.first_point.as_ref())
            .cross(&(self.first_point.as_ref() - x))
            .len();

        self.first_normal.as_ref() * area_bcx + self.second_normal.as_ref() * area_cax + self.third_normal.as_ref() * area_abx
    }
}
