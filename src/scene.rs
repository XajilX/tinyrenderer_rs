use crate::{
    camera::Camera,
    light::{Light, BP_P},
    linalg::{Matrix4, Vector2, Vector3},
    model::Model,
    texture,
    utils::EPS,
};
use std::f32::consts::PI;

pub struct Scene {
    camera: Camera,
    models: Vec<Model>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(
                Vector3::new(0., 0., 0.),
                Vector3::new(0., 0., -1.).normalize(),
                Vector3::new(0., 1., 0.).normalize(),
                90. * PI / 180.,
                1.,
            ),
            models: vec![],
            lights: vec![],
        }
    }
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
    pub fn rasterize(&self, width: usize, height: usize, msaa: usize) -> Vec<u8> {
        assert!(msaa <= 16);
        assert!((width as f32 / height as f32 - self.camera.aspect).abs() < EPS);
        let msaa = if msaa <= 1 { 1 } else { msaa };
        let mut fb = vec![0u8; width * height * 3 * msaa * msaa];
        let mut zb = vec![f32::INFINITY; width * height * msaa * msaa];
        let viewport_mat = Matrix4 {
            v: [
                [width as f32 / 2., 0., 0., width as f32 / 2.],
                [0., height as f32 / 2., 0., height as f32 / 2.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        };

        let cmat =
            viewport_mat * self.camera.perspective_transform() * self.camera.camera_transform();
        for model in &self.models {
            for tr in model.iter() {
                let (p0, p1, p2) = (
                    cmat * tr.v[0].homo_point(),
                    cmat * tr.v[1].homo_point(),
                    cmat * tr.v[2].homo_point(),
                );
                let (pc0, pc1, pc2) = (p0.vec3_homo(), p1.vec3_homo(), p2.vec3_homo());
                let (l, r) = (
                    pc0.v[0].min(pc1.v[0]).min(pc2.v[0]).max(0.) as usize,
                    pc0.v[0].max(pc1.v[0]).max(pc2.v[0]).min(width as f32 - 1.) as usize,
                );
                let (t, b) = (
                    pc0.v[1].min(pc1.v[1]).min(pc2.v[1]).max(0.) as usize,
                    pc0.v[1].max(pc1.v[1]).max(pc2.v[1]).min(height as f32 - 1.) as usize,
                );
                for xpx in l..=r {
                    let mut c_crs = 0;
                    for ypx in t..=b {
                        let mut c_smp = 0;
                        for kx in 0..msaa {
                            for ky in 0..msaa {
                                let ps = Vector2::new(
                                    (2 * xpx * msaa + 2 * kx + 1) as f32 / (2. * msaa as f32),
                                    (2 * ypx * msaa + 2 * ky + 1) as f32 / (2. * msaa as f32),
                                );
                                let buf_idx = (xpx + (height - ypx - 1) * width) * msaa * msaa
                                    + ky * msaa
                                    + kx;
                                let (pf0, pf1, pf2) = (
                                    Vector2::new(pc0.v[0], pc0.v[1]),
                                    Vector2::new(pc1.v[0], pc1.v[1]),
                                    Vector2::new(pc2.v[0], pc2.v[1]),
                                );
                                let (l0, l1, l2) = (pf1 - pf0, pf2 - pf1, pf0 - pf2);
                                let (v1, v2) = (ps - pf1, ps - pf2);
                                let (af, bf) =
                                    (l1.cross(v1) / l1.cross(-l0), l2.cross(v2) / l2.cross(-l1));

                                if af < 0. || bf < 0. || af + bf > 1. {
                                    continue;
                                }

                                //  perspective correction
                                let zn =
                                    1. / (af / p0.v[3] + bf / p1.v[3] + (1. - af - bf) / p2.v[3]);
                                let (a, b, c) = (
                                    af * zn / p0.v[3],
                                    bf * zn / p1.v[3],
                                    (1. - af - bf) * zn / p2.v[3],
                                );
                                let psz = a * pc0.v[2] + b * pc1.v[2] + c * pc2.v[2];
                                if psz > zb[buf_idx] {
                                    continue;
                                }
                                zb[buf_idx] = psz;
                                c_smp += 1;

                                let pos = tr.v[0] * a + tr.v[1] * b + tr.v[2] * c;
                                let norm = (tr.n[0] * a + tr.n[1] * b + tr.n[2] * c).normalize();
                                let uv = tr.uv[0] * a + tr.uv[1] * b + tr.uv[2] * c;
                                let clr = if let Some(texture) = &tr.texture {
                                    let uc = texture.at_uv(uv.v[0], uv.v[1]);
                                    Vector3::new(uc[0] as f32, uc[1] as f32, uc[2] as f32)
                                } else {
                                    Vector3::new(255., 255., 255.)
                                } / 255.;

                                //  Blinn-Phong shading
                                let mut liv = Vector3::new(0., 0., 0.);
                                let ambient = Vector3::new(0.03, 0.03, 0.03);
                                for light in &self.lights {
                                    match light {
                                        &Light::Point { pos: lp, li } => {
                                            let dist = (lp - pos).norm();
                                            let id2 = 1. / (dist * dist);
                                            let l = (lp - pos).normalize();
                                            let v = (self.camera.pos - pos).normalize();
                                            let h = (l + v).normalize();
                                            let diff = clr * (li * id2) * norm.dot(l).max(0.);
                                            let spec = Vector3::new(1., 1., 1.)
                                                * (li * id2)
                                                * norm.dot(h).max(0.).powf(BP_P);
                                            liv += ambient + diff + spec;
                                        }
                                        &Light::Parallel { dir: ld, li } => {
                                            let l = ld.normalize();
                                            let v = (self.camera.pos - pos).normalize();
                                            let h = (l + v).normalize();
                                            let diff = clr * li * norm.dot(l).max(0.);
                                            let spec = Vector3::new(1., 1., 1.)
                                                * li
                                                * norm.dot(h).max(0.).powf(BP_P);
                                            liv += ambient + diff + spec;
                                        }
                                    }
                                }

                                for m in 0..3 {
                                    fb[buf_idx * 3 + m] = (liv.v[m].max(0.).min(1.) * 255.) as u8;
                                }
                            }
                        }

                        if c_smp > 0 {
                            c_crs += 1;
                        } else if c_crs > 0 {
                            break;
                        }
                    }
                }
            }
        }

        if msaa <= 1 {
            fb
        } else {
            let mut fb_ret = vec![0u8; width * height * 3];
            for i in 0..width {
                for j in 0..height {
                    let mut px = [0., 0., 0.];
                    for k in 0..msaa {
                        for l in 0..msaa {
                            let idx = (i * msaa + k) + (j * msaa + l) * width * msaa;
                            for m in 0..3 {
                                px[m] += fb[idx * 3 + m] as f32;
                            }
                        }
                    }
                    for m in 0..3 {
                        px[m] /= (msaa * msaa) as f32;
                        fb_ret[(i + j * width) * 3 + m] = px[m] as u8;
                    }
                }
            }
            fb_ret
        }
    }
}
