use super::vector::Vector3D;

#[derive(PartialEq)]
pub struct Ray {
    pub from: Vector3D,
    pub direction: Vector3D,
    pub inside: bool,
}

impl Ray {
    fn propagate(&mut self, distance: f64) {
        self.from += &(&self.direction * distance);
    }
}
