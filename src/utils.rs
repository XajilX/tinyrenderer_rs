use crate::linalg::Vector2;

pub const EPS: f32 = 1e-4;

#[allow(unused)]
pub fn filter(p: Vector2, v: f32) -> (Vector2, f32, f32, f32, f32) {
    let p1 = Vector2::new(p.v[0].floor(), p.v[1].floor());
    let delta = p - p1;
    let (t, s) = (delta.v[0], delta.v[1]);
    (
        p1,
        v * (1. - t) * (1. - s),
        v * t * (1. - s),
        v * (1. - t) * s,
        v * t * s,
    )
}

pub fn barycentric_2d(tri: [Vector2; 3], p: Vector2) -> (f32, f32, f32) {
    let (l0, l1, l2) = (tri[1] - tri[0], tri[2] - tri[1], tri[0] - tri[2]);
    let (v1, v2) = (p - tri[1], p - tri[2]);
    let (a, b) = (l1.cross(v1) / l1.cross(-l0), l2.cross(v2) / l2.cross(-l1));
    (a, b, 1. - a - b)
}
