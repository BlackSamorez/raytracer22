use super::vector::Vector3D;

#[derive(PartialEq, Clone)]
pub struct Intersection {
    pub position: Vector3D,
    pub normal: Vector3D,
    pub distance: f64,
}
