use crate::linalg::Vector2;

pub const EPS: f32 = 1e-4;

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
