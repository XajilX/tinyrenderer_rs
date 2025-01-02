use crate::linalg::Vector3;

pub const BP_P: f32 = 160.;

pub enum Light {
    Parallel { dir: Vector3, li: Vector3 },
    Point { pos: Vector3, li: Vector3 },
}
