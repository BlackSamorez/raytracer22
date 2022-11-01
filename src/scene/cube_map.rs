use std::path::Path;

use image::RgbImage;

use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector3D;

enum StrongestDirection {
    Front,
    Back,
    Top,
    Bottom,
    Right,
    Left,
}

pub struct CubeMap {
    img: RgbImage,
}

impl CubeMap {
    pub fn new(image_path: &Path) -> Self {
        CubeMap { img: image::open(image_path).unwrap().as_rgb8().unwrap().to_owned() }
    }


    fn get_strongest_direction(ray: &Ray) -> StrongestDirection {
        let direction = &ray.direction;
        let abs = vec![direction.x.abs(), direction.y.abs(), direction.z.abs()];

        match abs.iter().enumerate().max_by(|(_, a), (_, b)| a.total_cmp(b)).map(|(pos, value)| (pos, *value > 0.0)) {
            Some((0, true)) => StrongestDirection::Front,
            Some((0, false)) => StrongestDirection::Back,
            Some((1, true)) => StrongestDirection::Top,
            Some((1, false)) => StrongestDirection::Bottom,
            Some((2, true)) => StrongestDirection::Right,
            Some((2, false)) => StrongestDirection::Left,
            _ => panic!("Error getting strongest direction"),
        }
    }

    pub fn trace(&self, ray: &Ray) -> Vector3D {
        let side_size = (self.img.width() / 4) as usize;
        let direction = &ray.direction;
        let abs = vec![direction.x.abs(), direction.y.abs(), direction.z.abs()];

        let x: u32;
        let y: u32;
        match abs.iter().enumerate().max_by(|(_, a), (_, b)| a.total_cmp(b)) {
            Some((0, &v)) if v >= 0.0 => {
                let scaled_direction = direction / v;
                x = (((3 * side_size) / 2) as f64 + side_size as f64 * scaled_direction.z / 2.0) as u32;
                y = (((3 * side_size) / 2) as f64 + side_size as f64 * -scaled_direction.y / 2.0) as u32;
            }
            Some((0, &v)) if v < 0.0 => {
                let scaled_direction = direction / v;
                x = (((7 * side_size) / 2) as f64 + side_size as f64 * -scaled_direction.z / 2.0) as u32;
                y = (((3 * side_size) / 2) as f64 + side_size as f64 * -scaled_direction.y / 2.0) as u32;
            }
            Some((1, &v)) if v >= 0.0 => {
                let scaled_direction = direction / v;
                x = (((3 * side_size) / 2) as f64 + side_size as f64 * scaled_direction.z / 2.0) as u32;
                y = (((1 * side_size) / 2) as f64 + side_size as f64 * scaled_direction.x / 2.0) as u32;
            }
            Some((1, &v)) if v < 0.0 => {
                let scaled_direction = direction / v;
                x = (((3 * side_size) / 2) as f64 + side_size as f64 * scaled_direction.z / 2.0) as u32;
                y = (((5 * side_size) / 2) as f64 + side_size as f64 * -scaled_direction.x / 2.0) as u32;
            }
            Some((2, &v)) if v >= 0.0 => {
                let scaled_direction = direction / v;
                x = (((5 * side_size) / 2) as f64 + side_size as f64 * -scaled_direction.x / 2.0) as u32;
                y = (((3 * side_size) / 2) as f64 + side_size as f64 * -scaled_direction.y / 2.0) as u32;
            }
            Some((3, &v)) if v < 0.0 => {
                let scaled_direction = direction / v;
                x = (((1 * side_size) / 2) as f64 + side_size as f64 * scaled_direction.x / 2.0) as u32;
                y = (((3 * side_size) / 2) as f64 + side_size as f64 * -scaled_direction.y / 2.0) as u32;
            }
            _ => panic!("Error getting strongest direction")
        }

        let color = self.img.get_pixel(x, y).0;

        Vector3D { x: color[0] as f64, y: color[1] as f64, z: color[2] as f64 }
    }
}

