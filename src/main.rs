use image::Rgb;
use nalgebra_glm as glm;
use glm::TVec3;
use anyhow::Result;

use std::path::Path;

mod ray;
mod scene;
mod sphere;
mod triangle;
mod stl_model;
use sphere::Sphere;
use triangle::Triangle;
use stl_model::StlModel;

type Vec3 = TVec3<f64>;

fn main() -> Result<()> {
    let scene = scene::Scene{
        width: 640,
        height: 480,
        camera: scene::Camera{
            viewpoint: Vec3::new(4.0, 3.0, 6.0),
            w: 8.0,
            h: 6.0,
        },
        objs: vec![
            Box::new(Sphere{
                center: Vec3::new(6.0, 4.0, -3.0),
                radius: 2.0,
                color: Rgb([255, 0, 0]),
            }),
            Box::new(Sphere{
                center: Vec3::new(2.0, 2.0, -3.0),
                radius: 1.3,
                color: Rgb([0, 0, 255]),
            }),
            Box::new(Triangle::new(
                [Vec3::new(2.0, 2.0, -2.0), Vec3::new(6.0, 4.0, -2.0), Vec3::new(6.0, 2.0, -2.0)],
                Rgb([0, 255, 0]),
                true,
            )),
            Box::new(StlModel::new(Path::new("./models/test.stl"), Rgb([0, 0, 255]))?),
        ],
    };

    let img = scene.render();

    img.save("out.png").unwrap();
    Ok(())
}
