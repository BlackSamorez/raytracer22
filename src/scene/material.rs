use crate::geometry::vector::Vector3D;

pub struct Material {
    pub name: String,
    pub ambient_color: Vector3D,
    pub diffuse_color: Vector3D,
    pub specular_color: Vector3D,
    pub intensity: Vector3D,
    pub specular_exponent: f64,
    pub refraction_index: f64,
    pub albedo: Vector3D,
}

impl std::default::Default for Material {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            ambient_color: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            diffuse_color: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            specular_color: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            intensity: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            specular_exponent: 0.0,
            refraction_index: 1.0,
            albedo: Vector3D { x: 1.0, y: 0.0, z: 0.0 },
        }
    }
}