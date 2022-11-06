use std::path::Path;

use cube_map::CubeMap;
use light::Light;
use object::Object;

pub mod cube_map;
pub mod light;
pub mod material;
pub mod object;
mod reader;

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub cube_map: Option<CubeMap>,
}

impl Scene {
    pub fn try_read(file_path: &Path) -> anyhow::Result<Self> {
        match reader::read_scene(file_path) {
            Ok(scene) => Ok(scene),
            Err(err) => Err(err.context(format!(", reading scene from {}", file_path.display()))),
        }
    }
}
