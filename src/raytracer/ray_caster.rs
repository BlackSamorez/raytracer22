use std::fs::File;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector3D;
use crate::geometry::EPSILON;

#[derive(Serialize, Deserialize)]
pub struct RayCasterConfig {
    pub height: usize,
    pub width: usize,
    pub fov: f64,
    pub look_from: Vector3D,
    pub look_to: Vector3D,
}

pub struct RayCaster {
    pub height: usize,
    pub width: usize,

    origin: Vector3D,
    backward_unit: Vector3D,
    right_unit: Vector3D,
    up_unit: Vector3D,
}

impl RayCaster {
    pub fn new(config_path: &Path) -> Self {
        let config: RayCasterConfig =
            serde_json::from_reader(File::open(config_path).unwrap_or_else(|_| {
                panic!("Couldn't open ray caster config: {}", config_path.display())
            }))
            .unwrap();

        let mut backward_unit = &config.look_from - &config.look_to;
        backward_unit.normalize();

        let mut right_unit = Vector3D::from([0., 1., 0.]).cross(&backward_unit);
        let mut up_unit;
        if right_unit.len() < EPSILON {
            right_unit = Vector3D::from([1., 0., 0.]);
            up_unit = Vector3D::from([0., 0., 1.]);
        } else {
            right_unit.normalize();
            up_unit = right_unit.cross(&backward_unit);
            up_unit.normalize();
        }

        let pixel_size = 2. * (config.fov / 2.).tan() / (config.height as f64);

        up_unit *= pixel_size;
        right_unit *= pixel_size;

        RayCaster {
            height: config.height,
            width: config.width,
            origin: config.look_from,
            backward_unit,
            right_unit,
            up_unit,
        }
    }

    pub fn cast_ray(&self, x: usize, y: usize) -> Ray {
        let mut direction = &self.right_unit * ((2 * x - self.width + 1) / 2) as f64
            + &self.up_unit * ((2 * y - self.height + 1) / 2) as f64
            - &self.backward_unit;
        direction.normalize();
        Ray {
            from: self.origin.clone(),
            direction,
        }
    }
}
