mod camera;
mod light;
mod linalg;
mod model;
mod scene;
mod texture;
mod triangle;
mod utils;

use camera::Camera;
use image::RgbImage;
use light::Light;
use linalg::transform::Transform;
use model::Model;
use scene::Scene;
use std::{error::Error, f32::consts::PI};

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

fn main() -> Result<(), Box<dyn Error>> {
    let mut scene = Scene::new();
    let camera = Camera::new(
        vect![-1., 0.5, -1.],
        vect![1., 0., 1.],
        vect![0., 1., 0.],
        90. * PI / 180.,
        1.,
    );
    scene.set_camera(camera);
    let mut model = Model::open("test/spot_triangulated_good.obj")?;
    let mat = Transform::new()
        .translation(vect![0.5, 0., 0.5])
        .rotation(vect![0., 1., 0.], PI / 4.)
        .mat();
    model.apply(mat);
    model.load_texture("test/spot_texture.png")?;
    scene.add_model(model);
    let light1 = Light::Point {
        pos: vect![0., 3., 0.],
        li: vect![10., 10., 10.],
    };
    let light2 = Light::Parallel {
        dir: vect![1., 0., 0.],
        li: vect![0.8, 0.8, 0.8],
    };
    scene.add_light(light1);
    scene.add_light(light2);
    let buf = scene.rasterize(WIDTH, HEIGHT, 4);
    let img = RgbImage::from_raw(WIDTH as u32, HEIGHT as u32, buf).unwrap();
    img.save("test/test.png").unwrap();
    Ok(())
}
