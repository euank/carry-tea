use image::Rgb;
use nalgebra_glm as glm;

use glm::TVec3;

mod ray;
mod scene;

type Vec3 = TVec3<f64>;

fn main() {
    let scene = scene::Scene{
        width: 640,
        height: 480,
        camera: scene::Camera{
            dir: Vec3::new(0.0, 0.0, -1.0),
            pos: Vec3::new(0.0, 0.0, 0.0),
            w: 8.0,
            h: 6.0,
        },
        objs: vec![Box::new(scene::Sphere{
            center: Vec3::new(0.0, 0.0, -3.0),
            radius: 1.3,
            color: Rgb([0, 255, 0]),
        })],
    };

    let img = scene.render();

    img.save("out.png").unwrap();
}
