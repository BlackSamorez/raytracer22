use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use log::{debug, info};

use crate::geometry::vector::Vector3D;
use crate::scene::light::Light;
use crate::scene::material::Material;
use crate::scene::object::PseudoObject;
use crate::scene::CubeMap;
use crate::scene::Scene;

fn read_point(triplet: &[&str]) -> Result<Vector3D> {
    if triplet.len() != 3 {
        return Err(anyhow!("Point must have 3 numbers"));
    }

    let vec = Vector3D {
        x: match triplet[0].parse() {
            Ok(val) => val,
            Err(err) => return Err(anyhow::Error::from(err).context("reading x")),
        },
        y: match triplet[1].parse() {
            Ok(val) => val,
            Err(err) => return Err(anyhow::Error::from(err).context("reading y")),
        },
        z: match triplet[2].parse() {
            Ok(val) => val,
            Err(err) => return Err(anyhow::Error::from(err).context("reading z")),
        },
    };
    Ok(vec)
}

fn read_indices_pairs(face_elements: &[&str]) -> Vec<(isize, isize)> {
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

fn read_materials(materials_path: &Path) -> Result<HashMap<String, Arc<Material>>> {
    debug!("Reading materials from {}", materials_path.display());
    let file = File::open(materials_path)
        .unwrap_or_else(|_| panic!("Couldn't open materials file: {}", materials_path.display()));
    let reader = BufReader::new(file);

    let mut materials: HashMap<String, Arc<Material>> = HashMap::new();
    let mut current_material = Material::default();
    let mut current_material_started = false;
    for (n, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        let result = match tokens.as_slice() {
            ["newmtl", name] => {
                if current_material_started {
                    // Save previous material
                    materials.insert(current_material.name.clone(), Arc::new(current_material));
                }
                current_material = Material::default();
                current_material_started = true;
                current_material.name = (*name).to_owned();
                anyhow::Ok(())
            }
            ["Ka", body @ ..] => match read_point(body) {
                Ok(vec) => {
                    current_material.ambient_color = vec;
                    Ok(())
                }
                Err(err) => Err(err.context("reading ambient_color")),
            },
            ["Kd", body @ ..] => match read_point(body) {
                Ok(vec) => {
                    current_material.diffuse_color = vec;
                    Ok(())
                }
                Err(err) => Err(err.context("reading diffuse_color")),
            },
            ["Ks", body @ ..] => match read_point(body) {
                Ok(vec) => {
                    current_material.specular_color = vec;
                    Ok(())
                }
                Err(err) => Err(err.context("reading specular_color")),
            },
            ["Ke", body @ ..] => match read_point(body) {
                Ok(vec) => {
                    current_material.intensity = vec;
                    Ok(())
                }
                Err(err) => Err(err.context("reading intensity")),
            },
            ["Ns", exponent] => match exponent.parse() {
                Ok(exp) => {
                    current_material.specular_exponent = exp;
                    Ok(())
                }
                Err(err) => Err(anyhow::Error::from(err).context("reading specular_exponent")),
            },
            ["Ni", index] => match index.parse() {
                Ok(exp) => {
                    current_material.refraction_index = exp;
                    Ok(())
                }
                Err(err) => Err(anyhow::Error::from(err).context("reading refraction_index")),
            },
            ["al", body @ ..] => match read_point(body) {
                Ok(vec) => {
                    current_material.albedo = vec;
                    Ok(())
                }
                Err(err) => Err(err.context("reading albedo")),
            },
            ["illum", ..] => Ok(()),
            [smt, ..] if smt.starts_with('#') => Ok(()),
            _ => Err(anyhow!("Unknown .mtl key")),
        };

        match result {
            Ok(()) => {}
            Err(err) => {
                return Err(err.context(format!(
                    "on line {} of {}",
                    n + 1,
                    materials_path.display()
                )))
            }
        }
    }
    if current_material_started {
        // Save previous material
        materials.insert(current_material.name.clone(), Arc::new(current_material));
    }

    debug!("Done reading materials from {}", materials_path.display());
    Ok(materials)
}

fn get_index(read_idx: isize, length: usize) -> usize {
    if read_idx > 0 {
        (read_idx - 1) as usize
    } else {
        (length as isize + read_idx) as usize
    }
}

fn get_object_normal(
    vertices: &[Vector3D],
    normals: &[Vector3D],
    idx: &[(isize, isize)],
) -> Vector3D {
    let first_vertex = &vertices[get_index(idx[0].0, vertices.len())];
    let second_vertex = &vertices[get_index(idx[1].0, vertices.len())];
    let third_vertex = &vertices[get_index(idx[2].0, vertices.len())];
    let unoriented_normal = (second_vertex - first_vertex)
        .cross(third_vertex - first_vertex)
        .normalize();
    if &unoriented_normal * &normals[get_index(idx[0].0, vertices.len())] < 0.0 {
        -unoriented_normal
    } else {
        unoriented_normal
    }
}

fn read_object(
    body: &[&str],
    vertices: &[Vector3D],
    read_normals: &[Vector3D],
    assigned_normals: &mut [Vector3D],
    material: &Arc<Material>,
) -> Result<Vec<PseudoObject>> {
    let indices_pairs = read_indices_pairs(body);
    if indices_pairs.len() < 3 {
        return Err(anyhow!("Object can't have less than 3 vertices"));
    }

    let no_normals = indices_pairs.iter().all(|(_, normal)| *normal == 0);
    let all_normals = indices_pairs.iter().all(|(_, normal)| *normal != 0);

    if !no_normals && !all_normals {
        return Err(anyhow!(
            "Ether all vertices should have normals or none should"
        ));
    }

    let mut pseudo_objects = vec![];

    let first_vertex_idx = get_index(indices_pairs[0].0, vertices.len());
    for i in 1..indices_pairs.len() - 1 {
        let second_vertex_idx = get_index(indices_pairs[i].0, vertices.len());
        let third_vertex_idx = get_index(indices_pairs[i + 1].0, vertices.len());
        pseudo_objects.push(PseudoObject {
            material: Arc::clone(material),
            first_vertex_idx,
            second_vertex_idx,
            third_vertex_idx,
        })
    }

    let object_normal = get_object_normal(vertices, assigned_normals, indices_pairs.as_slice());
    for (vertex_idx, normal_idx) in indices_pairs.iter() {
        assigned_normals[get_index(*vertex_idx, vertices.len())] += if all_normals {
            read_normals[get_index(*normal_idx, read_normals.len())].clone()
        } else {
            object_normal.clone()
        }
    }

    Ok(pseudo_objects)
}

fn read_light(body: &[&str]) -> Result<Light> {
    if body.len() != 6 {
        return Err(anyhow!("Light must have 6 points, got {}", body.len()));
    }

    let light = Light {
        position: match read_point(&body[..3]) {
            Ok(position) => position,
            Err(err) => return Err(err.context("reading position")),
        },
        intensity: match read_point(&body[..3]) {
            Ok(intensity) => intensity,
            Err(err) => return Err(err.context("reading intensity")),
        },
    };

    Ok(light)
}

pub fn read_scene(file_path: &Path) -> Result<Scene> {
    info!("Reading scene from {}", file_path.display());
    let file = File::open(file_path)
        .unwrap_or_else(|_| panic!("Couldn't open scene file: {}", file_path.display()));
    let reader = BufReader::new(file);

    let mut vertices = vec![];
    let mut read_normals = vec![];
    let mut assigned_normals = vec![];
    let mut materials = HashMap::new();
    let mut pseudo_objects = vec![];
    let mut lights = vec![];
    let mut cube_map = None;
    let mut current_material: Arc<Material> = Arc::new(Material::default());

    for (n, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        let result = match tokens.as_slice() {
            ["v", body @ ..] => match read_point(body) {
                Ok(normal) => {
                    vertices.push(normal);
                    assigned_normals.push(Vector3D::default());
                    anyhow::Ok(())
                }
                Err(err) => Err(err.context("reading vertex")),
            },
            ["vt", ..] => Ok(()),
            ["vn", body @ ..] => match read_point(body) {
                Ok(normal) => {
                    read_normals.push(normal);
                    Ok(())
                }
                Err(err) => Err(err.context("reading normal")),
            },
            ["f", body @ ..] => match read_object(
                body,
                vertices.as_slice(),
                read_normals.as_slice(),
                assigned_normals.as_mut_slice(),
                &current_material,
            ) {
                Ok(ref mut read_pseudo_objects) => {
                    pseudo_objects.append(read_pseudo_objects);
                    Ok(())
                }
                Err(err) => Err(err.context("reading object")),
            },
            ["mtllib", mtl_filename] => {
                match read_materials(&file_path.parent().unwrap().join(Path::new(mtl_filename))) {
                    Ok(read_materials) => {
                        materials = read_materials;
                        Ok(())
                    }
                    Err(err) => Err(err.context("reading underlying .mtl")),
                }
            }
            ["usemtl", mtl_name] => {
                current_material = Arc::clone(&materials[*mtl_name]);
                Ok(())
            }
            ["P", body @ ..] => match read_light(body) {
                Ok(light) => {
                    lights.push(light);
                    Ok(())
                }
                Err(err) => Err(err.context("reading light")),
            },
            ["Sky", _, _, sky_filename] => {
                cube_map = Some(CubeMap::new(
                    &file_path.parent().unwrap().join(Path::new(sky_filename)),
                ));
                Ok(())
            }
            [smt, ..] if smt.starts_with('#') => Ok(()),
            ["g", ..] => Ok(()),
            ["s", ..] => Ok(()),
            _ => Err(anyhow!("Unknown .obj key")),
        };

        match result {
            Ok(()) => {}
            Err(err) => {
                return Err(err.context(format!("on line {} of {}", n + 1, file_path.display())))
            }
        }
    }

    let final_normals: Vec<Vector3D> = assigned_normals
        .iter_mut()
        .map(|normal| normal.clone().normalize())
        .collect();

    info!("Done reading scene from {}", file_path.display());

    Ok(Scene {
        objects: pseudo_objects
            .iter()
            .map(|x| x.build_object(&vertices, &final_normals))
            .collect(),
        lights,
        cube_map,
    })
}
