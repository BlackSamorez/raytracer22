use crate::geometry::vector::Vector3D;


pub struct Material {
    name: String,
    ambient_color: Vector3D,
    diffuse_color: Vector3D,
    specular_color: Vector3D,
    intensity: Vector3D,
    specular_exponent: f64,
    refraction_index: f64,
    albedo: Vector3D,
}

impl std::default::Default for Material {
    fn default() -> Self {
        Self{
            name: "".to_owned(),
            ambient_color: Vector3D{x: 0.0, y: 0.0, z: 0.0},
            diffuse_color: Vector3D{x: 0.0, y: 0.0, z: 0.0},
            specular_color: Vector3D{x: 0.0, y: 0.0, z: 0.0},
            intensity: Vector3D{x: 0.0, y: 0.0, z: 0.0},
            specular_exponent: 0.0,
            refraction_index: 1.0,
            albedo: Vector3D{x: 1.0, y: 0.0, z: 0.0},
        }
    }
}