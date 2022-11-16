use std::path::Path;

use image::RgbImage;

use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector3D;

pub enum StrongestDirection {
    Front,
    Back,
    Top,
    Bottom,
    Right,
    Left,
}

fn get_strongest_direction(vector: &Vector3D) -> StrongestDirection {
    let abs = vec![vector.x.abs(), vector.y.abs(), vector.z.abs()];

    match abs
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(pos, _)| pos)
        .unwrap()
    {
        0 if vector.x > 0.0 => StrongestDirection::Front,
        0 => StrongestDirection::Back,
        1 if vector.y > 0.0 => StrongestDirection::Top,
        1 => StrongestDirection::Bottom,
        2 if vector.z > 0.0 => StrongestDirection::Right,
        2 => StrongestDirection::Left,
        _ => panic!("Error getting strongest direction"),
    }
}

pub struct CubeMap {
    img: RgbImage,
}

impl CubeMap {
    pub fn new(image_path: &Path) -> Self {
        CubeMap {
            img: image::open(image_path)
                .unwrap_or_else(|_| panic!("Couldn't open cube map {}", image_path.display()))
                .into_rgb8(),
        }
    }

    pub fn trace(&self, ray: &Ray) -> Vector3D {
        let side_size = (self.img.width() / 4) as usize;
        let direction = &ray.direction;

        let x: u32;
        let y: u32;
        match get_strongest_direction(direction) {
            StrongestDirection::Front => {
                let scaled_direction = direction / direction.x;
                x = (3. / 2. * side_size as f64 - side_size as f64 * -scaled_direction.z / 2.0)
                    as u32;
                y = (3. / 2. * side_size as f64 + side_size as f64 * -scaled_direction.y / 2.0)
                    as u32;
            }
            StrongestDirection::Back => {
                let scaled_direction = direction / -direction.x;
                x = (7. / 2. * side_size as f64 + side_size as f64 * -scaled_direction.z / 2.0)
                    as u32;
                y = (3. / 2. * side_size as f64 + side_size as f64 * -scaled_direction.y / 2.0)
                    as u32;
            }
            StrongestDirection::Top => {
                let scaled_direction = direction / direction.y;
                x = (3. / 2. * side_size as f64 + side_size as f64 * scaled_direction.z / 2.0)
                    as u32;
                y = (1. / 2. * side_size as f64 + side_size as f64 * scaled_direction.x / 2.0)
                    as u32;
            }
            StrongestDirection::Bottom => {
                let scaled_direction = direction / -direction.y;
                x = (3. / 2. * side_size as f64 + side_size as f64 * scaled_direction.z / 2.0)
                    as u32;
                y = (5. / 2. * side_size as f64 + side_size as f64 * -scaled_direction.x / 2.0)
                    as u32
                    - 1;
            }
            StrongestDirection::Right => {
                let scaled_direction = direction / direction.z;
                x = (5. / 2. * side_size as f64 + side_size as f64 * -scaled_direction.x / 2.0)
                    as u32;
                y = (3. / 2. * side_size as f64 + side_size as f64 * -scaled_direction.y / 2.0)
                    as u32;
            }
            StrongestDirection::Left => {
                let scaled_direction = direction / -direction.z;
                x = (1. / 2. * side_size as f64 + side_size as f64 * scaled_direction.x / 2.0)
                    as u32;
                y = (3. / 2. * side_size as f64 + side_size as f64 * -scaled_direction.y / 2.0)
                    as u32;
            }
        }

        let color = self.img.get_pixel(x, y).0;

        Vector3D {
            x: color[0] as f64 / 256.,
            y: color[1] as f64 / 256.,
            z: color[2] as f64 / 256.,
        }
    }
}
