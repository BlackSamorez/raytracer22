mod ray_caster;

use image::RgbImage;
use std::path::Path;
use std::sync::{Arc, MutexGuard, RwLock};

use crate::geometry::vector::Vector3D;
use ray_caster::RayCaster;

use crate::scene::Scene;

pub struct Raytracer {
    scene: Scene,
    ray_caster: RayCaster,
    data: Arc<RwLock<Vec<Vector3D>>>,
}

impl Raytracer {
    pub fn new(scene_path: &Path, ray_caster_path: &Path) -> Self {
        let ray_caster = RayCaster::new(ray_caster_path);
        Self {
            scene: Scene::new(scene_path),
            data: Arc::new(RwLock::new(vec![
                Vector3D::default();
                (ray_caster.width * ray_caster.height)
                    as usize
            ])),
            ray_caster,
        }
    }

    pub fn raytrace(&mut self) -> RgbImage {
        self.trace_full_image();
        self.get_rgb_image()
    }
}

impl Raytracer {
    fn get_pixel(&self, x: usize, y: usize) -> Vector3D {
        // Построчно слева направо снизу вверх
        self.data.read().unwrap()[x + self.ray_caster.width * y].clone()
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: Vector3D) {
        // Построчно слева направо снизу вверх
        *self
            .data
            .write()
            .unwrap()
            .get_mut(x + self.ray_caster.width * y)
            .unwrap() = pixel;
    }

    fn trace_ray(&self, x: usize, y: usize) -> Vector3D {
        Vector3D::from([100., 0., 0.])
    }

    fn trace_full_image(&mut self) {
        for x in 0..self.ray_caster.width {
            for y in 0..self.ray_caster.height {
                self.set_pixel(x, y, self.trace_ray(x, y));
            }
        }
    }

    fn get_rgb_image(&self) -> RgbImage {
        let bytes: Vec<u8> = self
            .data
            .read()
            .unwrap()
            .clone()
            .into_iter()
            .flat_map(|vec| [vec.x as u8, vec.y as u8, vec.z as u8])
            .collect();

        RgbImage::from_raw(
            self.ray_caster.width as u32,
            self.ray_caster.height as u32,
            bytes,
        )
        .unwrap()
    }
}
