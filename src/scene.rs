pub mod cube_map;
pub mod object;
pub mod material;
mod reader;

use crate::geometry::vector::Vector3D;
use crate::scene::object::Object;

use std::sync::Arc;
use cube_map::CubeMap;
use crate::scene::material::Material;


struct Scene {
    vertices: Vec<Arc<Vector3D>>,
    normals: Vec<Arc<Vector3D>>,
    materials: Vec<Arc<Material>>,

    pub objects: Vec<Object>,
    pub cube_map: Option<CubeMap>,
}



