use std::rc::Rc;

use crate::{
    linalg::{Vector2, Vector3},
    texture::Texture,
};

#[derive(Debug)]
pub struct Triangle {
    pub v: [Vector3; 3],
    pub n: [Vector3; 3],
    pub uv: [Vector2; 3],
    pub texture: Option<Rc<Texture>>,
}
