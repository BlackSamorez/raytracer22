mod illumination;
mod ray_caster;

use image::RgbImage;
use indicatif::ProgressIterator;
use std::borrow::Borrow;
use std::path::Path;
use std::sync::{Arc, RwLock};

use crate::geometry::vector::Vector3D;
use crate::raytracer::illumination::calculate_illumination;
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
        calculate_illumination(&self.ray_caster.cast_ray(x, y), &self.scene, 4)
    }

    fn trace_full_image(&mut self) {
        for x in (0..self.ray_caster.width).progress() {
            for y in 0..self.ray_caster.height {
                self.set_pixel(x, y, self.trace_ray(x, y));
            }
        }
    }

    fn get_rgb_image(&self) -> RgbImage {
        let max = self
            .data
            .read()
            .unwrap()
            .borrow()
            .iter()
            .flat_map(|vec| [vec.x, vec.y, vec.z])
            .max_by(|x, y| x.total_cmp(y))
            .unwrap();
        let bytes: Vec<u8> = self
            .data
            .read()
            .unwrap()
            .clone()
            .into_iter()
            .flat_map(|vec| {
                [
                    (vec.x / max * 255.) as u8,
                    (vec.y / max * 255.) as u8,
                    (vec.z / max * 255.) as u8,
                ]
            })
            .collect();

        RgbImage::from_raw(
            self.ray_caster.width as u32,
            self.ray_caster.height as u32,
            bytes,
        )
        .unwrap()
    }
}
