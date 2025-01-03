use super::{Matrix4, Vector3};

pub struct Transform {
    mat: Matrix4,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            mat: Matrix4::identity(),
        }
    }

    pub fn translation(mut self, pos: Vector3) -> Self {
        let m = Matrix4 {
            v: [
                [1., 0., 0., pos.v[0]],
                [0., 1., 0., pos.v[1]],
                [0., 0., 1., pos.v[2]],
                [0., 0., 0., 1.],
            ],
        };
        self.mat = m * self.mat;
        self
    }

    pub fn scale(mut self, scale: Vector3) -> Self {
        let m = Matrix4 {
            v: [
                [scale.v[0], 0., 0., 0.],
                [0., scale.v[1], 0., 0.],
                [0., 0., scale.v[2], 0.],
                [0., 0., 0., 1.],
            ],
        };
        self.mat = m * self.mat;
        self
    }
    pub fn rotation(mut self, axis: Vector3, ang: f32) -> Self {
        let axis = axis.normalize();
        let k = Matrix4 {
            v: [
                [0., -axis.v[2], axis.v[1], 0.],
                [axis.v[2], 0., -axis.v[0], 0.],
                [-axis.v[1], axis.v[0], 0., 0.],
                [0., 0., 0., 1.],
            ],
        };
        self.mat = self.mat + k * ang.sin() * self.mat + k * k * (1. - ang.cos()) * self.mat;
        self
    }

    pub fn mat(&self) -> Matrix4 {
        self.mat
    }
}
