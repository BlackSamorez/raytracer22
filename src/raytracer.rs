use std::borrow::Borrow;
use std::cmp::min;
use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use image::RgbImage;
use indicatif::{ProgressBar, ProgressIterator};
use log::info;
use threadpool::ThreadPool;

use ray_caster::RayCaster;

use crate::geometry::vector::Vector3D;
use crate::raytracer::illumination::{calculate_illumination, get_sky};
use crate::scene::Scene;

mod illumination;
mod ray_caster;

static NUM_WORKERS: usize = 8;
static CHUNK_SIZE: usize = 16;

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

    fn trace_section_of_image(
        lines_done: Arc<AtomicUsize>,
        scene: Arc<Scene>,
        image_buffer: Arc<RwLock<ImageBuffer>>,
        ray_caster: Arc<RayCaster>,
        section_number: usize,
        number_of_sections: usize,
    ) {
        let image_width = ray_caster.width;
        let section_width = (image_width - 1) / number_of_sections + 1;
        let start = section_width * section_number;
        let end = min(section_width * (section_number + 1), image_width);
        for x in start..end {
            for y in 0..ray_caster.height {
                let illumination =
                    calculate_illumination(&ray_caster.cast_ray(x, y), scene.borrow(), 10);
                Self::set_pixel(Arc::clone(&image_buffer), x, y, illumination);
            }
            lines_done.fetch_add(1, Release);
        }
    }

    fn start_worker_pool(
        lines_done: Arc<AtomicUsize>,
        scene: Arc<Scene>,
        image_buffer: Arc<RwLock<ImageBuffer>>,
        ray_caster: Arc<RayCaster>,
    ) -> ThreadPool {
        let width = ray_caster.width;
        let num_chunks = (width - 1) / CHUNK_SIZE + 1;

        let pool = ThreadPool::new(NUM_WORKERS);

        for chunk_idx in 0..num_chunks {
            pool.execute({
                let lines_done = Arc::clone(&lines_done);
                let scene = Arc::clone(&scene);
                let image_buffer = Arc::clone(&image_buffer);
                let ray_caster = Arc::clone(&ray_caster);
                move || {
                    Self::trace_section_of_image(
                        lines_done,
                        scene,
                        image_buffer,
                        ray_caster,
                        chunk_idx,
                        num_chunks,
                    );
                }
            });
        }
        pool
    }

    fn start_dumper_thread(
        lines_done: Arc<AtomicUsize>,
        lines_total: usize,
        image_buffer: Arc<RwLock<ImageBuffer>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            while lines_done.load(Acquire) != lines_total {
                thread::sleep(Duration::from_secs((10 * lines_total / 250) as u64)); // heuristic
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

        let lines_done = Arc::new(AtomicUsize::new(0));

        let workers_pool = Self::start_worker_pool(
            Arc::clone(&lines_done),
            Arc::clone(&self.scene),
            Arc::clone(&self.image_buffer),
            Arc::clone(&self.ray_caster),
        );
        let dumper_thread = Self::start_dumper_thread(
            Arc::clone(&lines_done),
            self.ray_caster.width,
            Arc::clone(&self.image_buffer),
        );

        info!("Tracing objects");

        let bar = ProgressBar::new(self.ray_caster.width as u64);
        bar.set_message("lines done");

        while lines_done.load(Relaxed) != self.ray_caster.width {
            bar.set_position(lines_done.load(Relaxed) as u64);
            thread::sleep(Duration::from_secs(1));
        }
        bar.set_position(self.ray_caster.width as u64);

        workers_pool.join();
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
