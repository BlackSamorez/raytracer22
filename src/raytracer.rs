use std::borrow::Borrow;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use image::RgbImage;
use indicatif::ProgressIterator;
use log::info;

use ray_caster::RayCaster;

use crate::geometry::vector::Vector3D;
use crate::raytracer::illumination::{calculate_illumination, get_sky};
use crate::scene::Scene;

mod illumination;
mod ray_caster;

struct ImageBuffer {
    width: usize,
    data: Vec<Vector3D>,
}

pub struct Raytracer {
    scene: Arc<Scene>,
    ray_caster: Arc<RayCaster>,
    image_buffer: Arc<RwLock<ImageBuffer>>,
}

impl Raytracer {
    pub fn new(scene_path: &Path, ray_caster_path: &Path) -> Self {
        let ray_caster = RayCaster::new(ray_caster_path);
        let scene = Arc::new(Scene::try_read(scene_path).unwrap());
        let image_buffer = Arc::new(RwLock::new(ImageBuffer {
            width: ray_caster.width,
            data: vec![Vector3D::default(); (ray_caster.width * ray_caster.height) as usize],
        }));
        let ray_caster = Arc::new(ray_caster);
        Self {
            scene,
            image_buffer,
            ray_caster,
        }
    }

    pub fn raytrace(&mut self) {
        self.trace_full_image_multiprocess_with_dumps();
    }
}

impl Raytracer {
    fn set_pixel(image: Arc<RwLock<ImageBuffer>>, x: usize, y: usize, pixel: Vector3D) {
        let width = image.read().unwrap().width;
        // Построчно слева направо снизу вверх
        *image.write().unwrap().data.get_mut(x + width * y).unwrap() = pixel;
    }

    fn trace_sky(
        scene: Arc<Scene>,
        image_buffer: Arc<RwLock<ImageBuffer>>,
        ray_caster: Arc<RayCaster>,
    ) {
        for x in (0..ray_caster.width).progress() {
            for y in 0..ray_caster.height {
                let sky_color = get_sky(&ray_caster.cast_ray(x, y), scene.borrow());
                Self::set_pixel(Arc::clone(&image_buffer), x, y, sky_color);
            }
        }
    }

    fn trace_full_image(
        scene: Arc<Scene>,
        image_buffer: Arc<RwLock<ImageBuffer>>,
        ray_caster: Arc<RayCaster>,
    ) {
        for x in (0..ray_caster.width).progress() {
            for y in 0..ray_caster.height {
                Self::set_pixel(
                    Arc::clone(&image_buffer),
                    x,
                    y,
                    calculate_illumination(&ray_caster.cast_ray(x, y), scene.borrow(), 10),
                );
            }
        }
    }

    fn start_worker_process(
        done: Arc<AtomicBool>,
        scene: Arc<Scene>,
        image_buffer: Arc<RwLock<ImageBuffer>>,
        ray_caster: Arc<RayCaster>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            Self::trace_full_image(scene, image_buffer, ray_caster);
            done.store(true, std::sync::atomic::Ordering::Release);
        })
    }

    fn start_dumper_thread(
        done: Arc<AtomicBool>,
        image_buffer: Arc<RwLock<ImageBuffer>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            while !done.load(std::sync::atomic::Ordering::Acquire) {
                thread::sleep(Duration::from_secs(10));
                info!("Saving image");
                Self::save_image(Arc::clone(&image_buffer), "intermediate.png");
            }
            info!("Saving image");
            Self::save_image(image_buffer, "intermediate.png");
        })
    }

    fn trace_full_image_multiprocess_with_dumps(&mut self) {
        info!("Tracing sky");
        Self::trace_sky(
            Arc::clone(&self.scene),
            Arc::clone(&self.image_buffer),
            Arc::clone(&self.ray_caster),
        );
        Self::save_image(Arc::clone(&self.image_buffer), "intermediate.png");

        let done = Arc::new(AtomicBool::new(false));

        let worker_thread = Self::start_worker_process(
            Arc::clone(&done),
            Arc::clone(&self.scene),
            Arc::clone(&self.image_buffer),
            Arc::clone(&self.ray_caster),
        );
        let dumper_thread =
            Self::start_dumper_thread(Arc::clone(&done), Arc::clone(&self.image_buffer));

        info!("Tracing objects");
        worker_thread.join().unwrap();
        dumper_thread.join().unwrap();
        info!("Tracing done");
    }

    fn get_rgb_image(image: Arc<RwLock<ImageBuffer>>) -> RgbImage {
        let max = image
            .read()
            .unwrap()
            .data
            .iter()
            .flat_map(|vec| [vec.x, vec.y, vec.z])
            .max_by(|x, y| x.total_cmp(y))
            .unwrap();
        let bytes: Vec<u8> = image
            .read()
            .unwrap()
            .data
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

        let width = image.read().unwrap().width as u32;
        let height = image.read().unwrap().data.len() as u32 / width;

        RgbImage::from_raw(width, height, bytes).unwrap()
    }

    fn save_image(image: Arc<RwLock<ImageBuffer>>, filename: &str) {
        Self::get_rgb_image(image)
            .save(Path::new(filename))
            .expect("Couldn't save image");
    }
}
