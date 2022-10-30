use crate::geometry::vector::Vector3D;
use crate::scene::Scene;
use crate::scene::CubeMap;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::sync::Arc;
use crate::scene::material::Material;
use crate::scene::object::Object;

fn read_point(triplet: &[&str]) -> Vector3D {
    assert!(triplet.len() >= 3);
    Vector3D{x: triplet[0].parse().unwrap(), y: triplet[1].parse().unwrap(), z: triplet[2].parse().unwrap()}
}

fn read_indices_pairs(face_elements: &[&str]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for &face_element in face_elements {
        match face_element.split('/').collect::<Vec<&str>>()[..] {
            [point, .., normal] => result.push((point.parse().unwrap(), normal.parse().unwrap())),
            [point] => result.push((point.parse().unwrap(), 0)),
            _ => panic!("Failed to parse indices"),
        }
    }

    result
}

pub fn read_scene(finelame: &str) -> Scene{
    let file = File::open(finelame).expect(&format!("Couldn't open scene file: {}", finelame));
    let reader = BufReader::new(file);

    let mut vertices: Vec<Arc<Vector3D>> = vec![];
    let mut normals: Vec<Arc<Vector3D>> = vec![];
    let mut dynamic_normals: Vec<(Arc<Vector3D>, usize)> = vec![];
    let mut materials: Vec<Arc<Material>> = vec![];
    let mut objects: Vec<Object> = vec![];
    let mut cube_map = None;

    for (n, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        match tokens.as_slice() {
            ["v", body @ ..] => vertices.push(Arc::new(read_point(body))),
            ["vt", ..] => continue,
            ["vn", body @ ..] => normals.push(Arc::new(read_point(body))),
            ["f", body @ ..] => {
                let indices_pairs = read_indices_pairs(body);
                assert!(indices_pairs.len() >= 3);

                let no_normals = indices_pairs.iter().all(|(_, normal)| *normal == 0);
                let all_normals = indices_pairs.iter().all(|(_, normal)| *normal != 0);
                assert!(no_normals || all_normals, "Either all point should have normals or none should");

                let first_pair = indices_pairs[0];
                for i in 1..indices_pairs.len()-1 {
                    let second_pair = indices_pairs[i];
                    let third_pair = indices_pairs[i + 1];

                    if no_normals {

                    } else {

                    }
                }
            },

            _ => panic!("Unknown key")
        }
    }

    Scene{vertices, normals, materials, objects, cube_map}
}