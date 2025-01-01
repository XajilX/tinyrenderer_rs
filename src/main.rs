mod camera;
mod linalg;
mod model;
mod texture;
mod triangle;
mod utils;

use camera::Camera;
use image::RgbImage;
use linalg::{Vector2, Vector3};
use std::f32::consts::PI;
use utils::filter;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() {
    let ori = Vector3::new(0., 0., 0.);
    let xi = Vector3::new(1., 0., 0.);
    let yi = Vector3::new(0., 1., 0.);
    let zi = Vector3::new(0., 0., 1.);
    let pos = Vector3::new(1., 1., 1.);
    let dir = Vector3::new(-1., -1., -1.).normalize();
    let up = Vector3::new(-1., 2., -1.).normalize();
    let camera = Camera::new(pos, dir, up, 90. * PI / 180., 1.);

    let trans = |v: Vector3| -> Vector3 {
        let vh = v.homo_point();
        let mat_cam = camera.camera_transform();
        let mat_pers = camera.perspective_transform(0.1, 30.);
        let mat_view = camera.viewport_transform(WIDTH as f32, HEIGHT as f32);
        (mat_view * (mat_pers * (mat_cam * vh))).vec3_homo()
    };

    let ori_t = trans(ori);
    let xi_t = trans(xi);
    let yi_t = trans(yi);
    let zi_t = trans(zi);

    let mut buf = vec![0u8; WIDTH * HEIGHT * 3];
    draw_line(
        &mut buf,
        Vector2::new(ori_t.v[0], ori_t.v[1]),
        Vector2::new(xi_t.v[0], xi_t.v[1]),
        [255, 0, 0],
    );
    draw_line(
        &mut buf,
        Vector2::new(ori_t.v[0], ori_t.v[1]),
        Vector2::new(yi_t.v[0], yi_t.v[1]),
        [0, 255, 0],
    );
    draw_line(
        &mut buf,
        Vector2::new(ori_t.v[0], ori_t.v[1]),
        Vector2::new(zi_t.v[0], zi_t.v[1]),
        [0, 0, 255],
    );
    let img = RgbImage::from_raw(WIDTH as u32, HEIGHT as u32, buf).unwrap();
    img.save("line.png").unwrap();
}

fn draw_line(img: &mut Vec<u8>, p1: Vector2, p2: Vector2, color: [u8; 3]) {
    let mut dir = p2 - p1;
    let step = dir.v[0].abs().max(dir.v[1].abs());
    dir *= 1. / step;
    let mut p = p1;
    let mut i = 0.;
    while i < step {
        for ch in 0..3 {
            let (p1, v1, v2, v3, v4) = filter(p, color[ch] as f32);
            let (x1, y1) = (p1.v[0] as usize, p1.v[1] as usize);
            let pb = (HEIGHT - y1 - 1) * WIDTH * 3 + x1 * 3 + ch;
            img[pb] = img[pb].saturating_add(v1 as u8);
            if x1 < WIDTH - 1 {
                img[pb + 3] = img[pb + 3].saturating_add(v2 as u8);
                if y1 < HEIGHT - 1 {
                    img[pb + 3 - 3 * WIDTH] = img[pb + 3 - WIDTH].saturating_add(v4 as u8);
                }
            }
            if y1 < HEIGHT - 1 {
                img[pb - 3 * WIDTH] = img[pb - 3 * WIDTH].saturating_add(v3 as u8);
            }
        }
        p += dir;
        i += 1.;
    }
}
