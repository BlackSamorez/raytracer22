use intersection::Intersection;
use polygon::Polygon;
use ray::Ray;
use vector::Vector3D;

pub mod intersection;
pub mod polygon;
pub mod ray;
pub mod vector;

pub static EPSILON: f64 = 1e-5;

pub fn get_intersection(ray: &Ray, polygon: &Polygon) -> Option<Intersection> {
    let first_edge = &polygon.second_point - &polygon.first_point;
    let second_edge = &polygon.third_point - &polygon.first_point;
    let h = ray.direction.cross(&second_edge);
    let a = &h * &first_edge;

    if a > -EPSILON && a < EPSILON {
        return None;
    }

    let f = 1.0 / a;
    let s = &ray.from - &polygon.first_point;
    let u = f * (&s * &h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross(&first_edge);
    let v = f * (&ray.direction * &q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = f * (&second_edge * &q);

    if t < EPSILON {
        return None;
    }

    let intersection_point = &ray.from + &ray.direction * t;
    let unoriented_normal = polygon.weighted_normal(&intersection_point);
    let normal = if &unoriented_normal * &ray.direction > 0.0 {
        -unoriented_normal.normalize()
    } else {
        unoriented_normal.normalize()
    };

    Some(Intersection {
        distance: (&intersection_point - &ray.from).len(),
        position: intersection_point,
        normal,
    })
}

pub fn refract(direction_in: &Vector3D, normal: &Vector3D, eta: f64) -> Option<Vector3D> {
    let c = -(normal * direction_in);
    let cosine2 = eta * eta * (1.0 - c * c);
    if cosine2 > 1.0 {
        None
    } else {
        Some(direction_in * eta + normal * (eta * c - cosine2.sqrt()))
    }
}

pub fn reflect(direction_in: &Vector3D, normal: &Vector3D) -> Vector3D {
    direction_in - normal * 2.0 * (normal * direction_in)
}
