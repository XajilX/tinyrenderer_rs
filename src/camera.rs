use crate::linalg::{Matrix4, Vector3};
use crate::utils::EPS;

const ZNR: f32 = 0.01;
const ZFR: f32 = 500.;

pub struct Camera {
    pub pos: Vector3,
    pub dir: Vector3,
    pub up: Vector3,
    pub fov_y: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn new(pos: Vector3, dir: Vector3, up: Vector3, fov_y: f32, aspect: f32) -> Self {
        assert!(dir.dot(up).abs() < EPS);
        Self {
            pos,
            dir: dir.normalize(),
            up: up.normalize(),
            fov_y,
            aspect,
        }
    }
    pub fn camera_transform(&self) -> Matrix4 {
        assert!(self.dir.dot(self.up).abs() < EPS);
        let v = self.dir.cross(self.up);
        let (kv, ku, kd) = (
            self.pos.dot(v),
            self.pos.dot(self.up),
            self.pos.dot(self.dir),
        );
        Matrix4 {
            v: [
                [v.v[0], v.v[1], v.v[2], -kv],
                [self.up.v[0], self.up.v[1], self.up.v[2], -ku],
                [-self.dir.v[0], -self.dir.v[1], -self.dir.v[2], kd],
                [0., 0., 0., 1.],
            ],
        }
    }
    pub fn perspective_transform(&self) -> Matrix4 {
        let fy = 1. / (self.fov_y / 2.).tan();
        let fx = fy / self.aspect;
        Matrix4 {
            v: [
                [fx, 0., 0., 0.],
                [0., fy, 0., 0.],
                [
                    0.,
                    0.,
                    (-ZNR - ZFR) / (ZFR - ZNR),
                    -2. * ZFR * ZNR / (ZFR - ZNR),
                ],
                [0., 0., -1., 0.],
            ],
        }
    }
}
