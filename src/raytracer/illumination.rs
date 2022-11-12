use std::sync::Arc;

use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector3D;
use crate::geometry::{EPSILON, get_intersection, reflect};
use crate::scene::material::Material;
use crate::scene::Scene;

fn find_all_nontrivial_collisions(ray: &Ray, scene: &Scene) -> Vec<(Intersection, Arc<Material>)> {
    let mut collisions = vec![];

    for object in scene.objects.iter() {
        match get_intersection(ray, &object.polygon)
            .map(|intersection| (intersection, Arc::clone(&object.material)))
        {
            None => continue,
            Some(collision) => collisions.push(collision),
        }
    }

    collisions
}

enum Collision {
    Sky,
    Polygon(Intersection, Arc<Material>),
}

fn get_the_collision(ray: &Ray, scene: &Scene) -> Collision {
    let nontrivial_collisions = find_all_nontrivial_collisions(ray, scene);
    if nontrivial_collisions.is_empty() {
        Collision::Sky
    } else {
        let (intersection, material) = (*nontrivial_collisions
            .iter()
            .min_by(|(a, _), (b, _)| a.distance.total_cmp(&b.distance))
            .unwrap())
        .clone();
        Collision::Polygon(intersection, material)
    }
}

pub fn calculate_illumination(ray: &Ray, scene: &Scene, ttl: usize) -> Vector3D {
    if ttl == 0 {
        return Vector3D::default();
    }

    let collision = get_the_collision(ray, scene);

    match collision {
        Collision::Sky => match scene.cube_map {
            None => Vector3D::default(),
            Some(ref cube_map) => cube_map.trace(ray),
        },

        Collision::Polygon(intersection, _) if ray.inside => {
            let phased_ray = Ray {
                from: intersection.position,
                direction: ray.direction.clone(),
                inside: !ray.inside,
            }.propagate(EPSILON);
            calculate_illumination(&phased_ray, scene, ttl - 1)
        }

        Collision::Polygon(intersection, _) => {
            let reflected_direction = reflect(&ray.direction, &intersection.normal);
            let reflected_ray = Ray {
                from: intersection.position,
                direction: reflected_direction,
                inside: !ray.inside,
            }.propagate(EPSILON);
            calculate_illumination(&reflected_ray, scene, ttl - 1)
        }
    }
}
