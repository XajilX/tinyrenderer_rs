mod camera;
mod linalg;
mod model;
mod scene;
mod texture;
mod triangle;
mod utils;

use camera::Camera;
use image::RgbImage;
use linalg::{Vector2, Vector3};
use model::Model;
use scene::Scene;
use std::{error::Error, f32::consts::PI};
use utils::filter;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut scene = Scene::new();
    let camera = Camera::new(
        Vector3::new(-2., 0.5, -2.),
        Vector3::new(1., 0., 1.),
        Vector3::new(0., 1., 0.),
        90. * PI / 180.,
        1.,
    );
    scene.set_camera(camera);
    let mut model = Model::open("spot_triangulated_good.obj")?;
    model.load_texture("spot_texture.png")?;
    scene.add_model(model);
    let buf = scene.rasterize(WIDTH, HEIGHT, 1);
    let img = RgbImage::from_raw(WIDTH as u32, HEIGHT as u32, buf).unwrap();
    img.save("test.png").unwrap();
    Ok(())
}

#[allow(dead_code)]
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
