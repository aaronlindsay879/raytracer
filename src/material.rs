use crate::colour::Colour;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub ambient_constant: Colour,
    pub diffuse_constant: Colour,
    pub specular_constant: Colour,
    pub shininess: f64,
}

impl Material {
    pub fn new(
        ambient_constant: impl Into<Colour>,
        diffuse_constant: impl Into<Colour>,
        specular_constant: impl Into<Colour>,
        shininess: f64,
    ) -> Self {
        Self {
            ambient_constant: ambient_constant.into(),
            diffuse_constant: diffuse_constant.into(),
            specular_constant: specular_constant.into(),
            shininess,
        }
    }
}
