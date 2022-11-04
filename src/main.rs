use crate::raytracer::Raytracer;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::path::Path;

mod geometry;
mod raytracer;
mod scene;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger)).map(|()| log::set_max_level(LevelFilter::Info))
}

fn main() {
    init().unwrap();

    let scene_path = std::env::args().nth(1).expect("No scene path given");
    let scene_path = Path::new(&scene_path);
    let ray_caster_path = std::env::args().nth(2).expect("No ray caster path given");
    let ray_caster_path = Path::new(&ray_caster_path);

    let mut raytracer = Raytracer::new(scene_path, ray_caster_path);
    let result = raytracer.raytrace();

    result
        .save(Path::new("result.png"))
        .expect("Couldn't save image");
}
