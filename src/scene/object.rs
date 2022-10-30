use crate::geometry::polygon::Polygon;
use crate::scene::material::Material;

use std::sync::Arc;

pub struct Object {
    pub polygon: Polygon,
    pub material: Arc<Material>,
}