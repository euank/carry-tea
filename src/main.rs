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
            viewpoint: Vec3::new(4.0, 3.0, 6.0),
            w: 8.0,
            h: 6.0,
        },
        objs: vec![
            Box::new(scene::Sphere{
                center: Vec3::new(6.0, 4.0, -2.0),
                radius: 2.0,
                color: Rgb([255, 0, 0]),
            }),
            Box::new(scene::Sphere{
                center: Vec3::new(2.0, 2.0, -2.0),
                radius: 1.3,
                color: Rgb([0, 0, 255]),
            }),
        ],
    };

    let img = scene.render();

    img.save("out.png").unwrap();
}
