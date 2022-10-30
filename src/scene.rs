pub mod cube_map;
pub mod object;
pub mod material;
mod reader;

use crate::geometry::vector::Vector3D;
use crate::scene::object::Object;

use std::rc::Rc;
use cube_map::CubeMap;
use crate::scene::material::Material;


pub struct Scene {
    materials: Vec<Rc<Material>>,

    pub objects: Vec<Object>,
    pub cube_map: Option<CubeMap>,
}



