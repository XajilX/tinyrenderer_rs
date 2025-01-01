use crate::linalg::{Matrix4, Vector3};
use crate::utils::EPS;

pub struct Camera {
    pos: Vector3,
    dir: Vector3,
    up: Vector3,
    fov_y: f32,
    aspect: f32,
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
                [-self.dir.v[0], -self.dir.v[1], -self.dir.v[2], -kd],
                [0., 0., 0., 1.],
            ],
        }
    }
    pub fn perspective_transform(&self, zmin: f32, zmax: f32) -> Matrix4 {
        assert!(zmin < zmax);
        assert!(zmin > 0.);
        let u = zmin * (self.fov_y / 2.).tan();
        let r = u * self.aspect;
        Matrix4 {
            v: [
                [zmin / r, 0., 0., 0.],
                [0., zmin / u, 0., 0.],
                [
                    0.,
                    0.,
                    (zmax + zmin) / (zmax - zmin),
                    -2. * zmax * zmin / (zmax - zmin),
                ],
                [0., 0., 1., 0.],
            ],
        }
    }
    pub fn viewport_transform(&self, width: f32, height: f32) -> Matrix4 {
        Matrix4 {
            v: [
                [width / 2., 0., 0., width / 2.],
                [0., height / 2., 0., height / 2.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
}
