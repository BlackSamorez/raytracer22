use super::vector::Vector3D;

#[derive(PartialEq)]
pub struct Polygon {
    pub first_point: Vector3D,
    pub second_point: Vector3D,
    pub third_point: Vector3D,
    pub first_normal: Vector3D,
    pub second_normal: Vector3D,
    pub third_normal: Vector3D,
}

impl Polygon {
    pub fn weighted_normal(&self, x: &Vector3D) -> Vector3D {
        let area_abx = (&self.second_point - &self.first_point)
            .cross(&(&self.second_point - x))
            .len();
        let area_bcx = (&self.third_point - &self.second_point)
            .cross(&(&self.third_point - x))
            .len();
        let area_cax = (&self.third_point - &self.first_point)
            .cross(&(&self.first_point - x))
            .len();

        &self.first_normal * area_bcx
            + &self.second_normal * area_cax
            + &self.third_normal * area_abx
    }
}
