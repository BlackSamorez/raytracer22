use crate::geometry::vector::Vector3D;
use crate::scene::Scene;
use crate::scene::CubeMap;
use crate::geometry::polygon::Polygon;
use crate::scene::material::Material;
use crate::scene::object::{Object, PseudoObject};

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::{AddAssign, Deref, DerefMut, Sub};
use std::rc::{Rc, Weak};


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

pub fn read_scene(filename: &str) -> Scene {
    let file = File::open(filename).expect(&format!("Couldn't open scene file: {}", filename));
    let reader = BufReader::new(file);

    let mut vertices: Vec<Vector3D> = vec![];
    let mut read_normals: Vec<Vector3D> = vec![];
    let mut assigned_normals: Vec<Vector3D> = vec![];
    let mut materials: Vec<Rc<Material>> = vec![];
    let mut pseudo_objects: Vec<PseudoObject> = vec![];
    let mut cube_map = None;
    let mut current_material: Option<Rc<Material>> = None;

    for (n, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        match tokens.as_slice() {
            ["v", body @ ..] => {
                vertices.push(read_point(body));
                assigned_normals.push(Vector3D::default());
            },
            ["vt", ..] => continue,
            ["vn", body @ ..] => read_normals.push(read_point(body)),
            ["f", body @ ..] => {
                let indices_pairs = read_indices_pairs(body);
                assert!(indices_pairs.len() >= 3);

                let no_normals = indices_pairs.iter().all(|(_, normal)| *normal == 0);
                let all_normals = indices_pairs.iter().all(|(_, normal)| *normal != 0);
                assert!(no_normals || all_normals, "Either all point should have normals or none should");

                let first_pair = indices_pairs[0];
                let first_point_idx = if first_pair.0 > 0 { first_pair.0 } else { vertices.len() + first_pair.0 };
                let first_point = vertices[first_point_idx].clone();
                for i in 1..indices_pairs.len()-1 {
                    let second_pair = indices_pairs[i];
                    let second_point_idx = if second_pair.0 > 0 { second_pair.0 } else { vertices.len() + second_pair.0 };
                    let second_point = vertices[second_point_idx].clone();

                    let third_pair = indices_pairs[i + 1];
                    let third_point_idx = if third_pair.0 > 0 { third_pair.0 } else { vertices.len() + third_pair.0 };
                    let third_point = vertices[third_point_idx].clone();

                    if no_normals {
                        let mut face_normal = (&second_point - &first_point).cross(&third_point - &first_point);
                        face_normal.normalize();

                        assigned_normals[first_point_idx] += &face_normal;
                        assigned_normals[second_point_idx] += &face_normal;
                        assigned_normals[third_point_idx] += &face_normal;
                    } else {
                        assigned_normals[first_point_idx] += &read_normals[if first_pair.1 > 0 { first_pair.1 } else { read_normals.len() + first_pair.1 }];
                        assigned_normals[second_point_idx] += &read_normals[if second_pair.1 > 0 { second_pair.1 } else { read_normals.len() + second_pair.1 }];
                        assigned_normals[third_point_idx] += &read_normals[if third_pair.1 > 0 { third_pair.1 } else { read_normals.len() + third_pair.1 }];
                    }

                    pseudo_objects.push(PseudoObject{
                        material: Rc::downgrade(current_material.as_ref().unwrap()),
                        first_point_idx,
                        second_point_idx,
                        third_point_idx,
                    })
                }
            },

            _ => panic!("Unknown key")
        }
    }

    for normal in assigned_normals.iter_mut() {
        normal.normalize();
    }

    Scene{
        materials,
        objects: pseudo_objects.iter().map(|x| x.build_object(&vertices, &assigned_normals)).collect(),
        cube_map
    }
}