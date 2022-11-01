use std::path::Path;
use std::rc::Rc;

use cube_map::CubeMap;
use light::Light;
use material::Material;
use object::Object;

pub mod cube_map;
pub mod object;
pub mod material;
pub mod light;
mod reader;

pub struct Scene {
    materials: Vec<Rc<Material>>,

    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub cube_map: Option<CubeMap>,
}

impl Scene {
    pub fn new(file_path: &Path) -> Self {
        reader::read_scene(file_path)
    }
}
