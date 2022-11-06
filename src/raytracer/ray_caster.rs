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
    forward: Vector3D,
    pixel_right: Vector3D,
    pixel_up: Vector3D,
}

impl RayCaster {
    pub fn new(config_path: &Path) -> Self {
        let config: RayCasterConfig =
            serde_json::from_reader(File::open(config_path).unwrap_or_else(|_| {
                panic!("Couldn't open ray caster config: {}", config_path.display())
            }))
            .unwrap();

        let mut forward = &config.look_to - &config.look_from;
        forward.normalize();

        let mut pixel_right = forward.cross(&Vector3D::from([0., 1., 0.]));
        pixel_right.normalize();

        let mut pixel_up = pixel_right.cross(&forward);
        pixel_up.normalize();

        let pixel_size = 2. * (config.fov / 2.).tan() / (config.height as f64);

        pixel_up *= pixel_size;
        pixel_right *= pixel_size;

        RayCaster {
            height: config.height,
            width: config.width,
            origin: config.look_from,
            forward,
            pixel_right,
            pixel_up,
        }
    }

    pub fn cast_ray(&self, x: usize, y: usize) -> Ray {
        let mut direction = &self.forward
            + &self.pixel_right * (x as f64 - self.width as f64 / 2.0)
            - &self.pixel_up * (y as f64 - self.height as f64 / 2.0);
        direction.normalize();
        Ray {
            from: self.origin.clone(),
            direction,
            inside: false,
        }
    }
}
