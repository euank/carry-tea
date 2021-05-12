use image::{RgbImage, Rgb};
use nalgebra_glm as glm;

use glm::TVec3;

mod ray;
mod scene;

use ray::Ray;

type Vec3 = TVec3<f64>;

fn color(r: &Ray) -> image::Rgb<u8> {
    let norm = r.dir.normalize();
    let t = 0.5 * (norm.y + 1.0);
    let v3 = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    // 0-1.0 f64 -> 0-255 u8
    let conv = |v: f64| -> u8 {
        (255.0 * v).clamp(0.0, 255.0) as u8
    };
    image::Rgb([conv(v3.x), conv(v3.y), conv(v3.z)])
}

fn main() {
    let width = 200;
    let height = 100;

    let ul = Vec3::new(-2.0, 1.0, -1.0);
    let horiz = Vec3::new(4f64, 0f64, 0f64);
    let vert = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut img = RgbImage::new(width, height);

    let scene = scene::Scene{
        objs: vec![Box::new(scene::Sphere{
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            color: image::Rgb([0, 255, 0]),
        })],
    };

    for x in 0..width {
        for y in 0..height {
            let u = x as f64 / width as f64;
            let v = y as f64 / width as f64;
            let r = Ray::new(origin, ul + u * horiz - v * vert);
            // background color
            let mut c = color(&r);
            // object color if we hit an object
            for obj in &scene.objs {
                match obj.is_hit(&r) {
                    Some(hit_c) => {
                        c = hit_c;
                    },
                    None => {},
                }
            }
            img.put_pixel(x, y, c);
        }
    }

    img.save("out.png").unwrap();
}
