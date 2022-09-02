use serde_tuple::Deserialize_tuple;

use crate::colour::Colour;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize_tuple)]
pub struct Material {
    pub ambient_constant: Colour,
    pub diffuse_constant: Colour,
    pub specular_constant: Colour,
    pub shininess: f64,
}
