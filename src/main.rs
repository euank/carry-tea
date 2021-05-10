use image::{RgbImage, Rgb};
use nalgebra_glm as glm;

use glm::TVec3;

mod ray;
use ray::Ray;

type Vec3 = TVec3<f64>;

fn color(r: &Ray) -> Vec3 {
    let norm = r.dir.normalize();
    let t = 0.5 * (norm.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let width = 200;
    let height = 100;

    let ul = Vec3::new(-2.0, 1.0, -1.0);
    let horiz = Vec3::new(4f64, 0f64, 0f64);
    let vert = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut img = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let u = x as f64 / width as f64;
            let v = y as f64 / width as f64;
            let r = Ray::new(origin, ul + u * horiz - v * vert);
            let c = color(&r);
            img.put_pixel(x, y, Rgb([
                    (255.0 * c.x).clamp(0.0, 255.0) as u8,
                    (255.0 * c.y).clamp(0.0, 255.0) as u8,
                    (255.0 * c.z).clamp(0.0, 255.0) as u8,
            ]));
        }
    }

    img.save("out.png").unwrap();
}
