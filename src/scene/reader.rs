use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::path::Path;
use std::rc::Rc;

use crate::geometry::vector::Vector3D;
use crate::scene::CubeMap;
use crate::scene::light::Light;
use crate::scene::material::Material;
use crate::scene::object::PseudoObject;
use crate::scene::Scene;

fn read_point(triplet: &[&str]) -> Vector3D {
    assert!(triplet.len() >= 3);
    Vector3D { x: triplet[0].parse().unwrap(), y: triplet[1].parse().unwrap(), z: triplet[2].parse().unwrap() }
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

fn read_materials(materials_path: &Path) -> HashMap<String, Rc<Material>> {
    let file = File::open(materials_path).unwrap_or_else(|_| panic!("Couldn't open materials file: {}", materials_path.to_str().unwrap()));
    let reader = BufReader::new(file);

    let mut materials: HashMap<String, Rc<Material>> = HashMap::new();
    let mut current_material = Material::default();
    let mut current_material_started = false;
    for (n, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match tokens.as_slice() {
            ["newmtl", name] => {
                if current_material_started { // Save previous material
                    materials.insert(current_material.name.clone(), Rc::new(current_material));
                }
                current_material = Material::default();
                current_material_started = true;
                current_material.name = (*name).to_owned();
            }
            ["Ka", body @ ..] => current_material.ambient_color = read_point(body),
            ["Kd", body @ ..] => current_material.diffuse_color = read_point(body),
            ["Ks", body @ ..] => current_material.specular_color = read_point(body),
            ["Ke", body @ ..] => current_material.intensity = read_point(body),
            ["Ns", exponent] => current_material.specular_exponent = exponent.parse().unwrap(),
            ["Ni", index] => current_material.refraction_index = index.parse().unwrap(),
            ["al", body @ ..] => current_material.albedo = read_point(body),
            _ => panic!("Unknown key"),
        }
    }

    materials
}

pub fn read_scene(file_path: &Path) -> Scene {
    let file = File::open(file_path).unwrap_or_else(|_| panic!("Couldn't open scene file: {}", file_path.to_str().unwrap()));
    let reader = BufReader::new(file);

    let mut vertices = vec![];
    let mut read_normals = vec![];
    let mut assigned_normals = vec![];
    let mut materials = HashMap::new();
    let mut pseudo_objects = vec![];
    let mut lights = vec![];
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
            }
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
                for i in 1..indices_pairs.len() - 1 {
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

                    pseudo_objects.push(PseudoObject {
                        material: Rc::downgrade(current_material.as_ref().unwrap()),
                        first_point_idx,
                        second_point_idx,
                        third_point_idx,
                    })
                }
            }
            ["mtllib", mtl_filename] => materials = read_materials(&file_path.parent().unwrap().join(Path::new(mtl_filename))),
            ["usemtl", mtl_name] => current_material = Some(Rc::clone(&materials[*mtl_name])),
            ["P", body @ ..] => {
                lights.push(Light {
                    position: read_point(&body[..3]),
                    intensity: read_point(&body[3..]),
                });
            }
            ["Sky", _, _, sky_filename] => cube_map = Some(CubeMap::new(&file_path.parent().unwrap().join(Path::new(sky_filename)))),
            _ => panic!("Unknown key")
        }
    }

    for normal in assigned_normals.iter_mut() {
        normal.normalize();
    }

    Scene {
        materials: materials.into_iter().map(|(_, v)| v).collect(),
        objects: pseudo_objects.iter().map(|x| x.build_object(&vertices, &assigned_normals)).collect(),
        lights,
        cube_map,
    }
}