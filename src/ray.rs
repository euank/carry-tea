use nalgebra_glm as glm;
use image::Rgb;

use glm::TVec3;

type Vec3 = TVec3<f64>;

pub(crate) struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray{origin, dir}
    }

    pub fn trace(&self, scene: &crate::scene::Scene) -> Rgb<u8> {
        for obj in &scene.objs {
            match obj.is_hit(&self) {
                Some(hit_c) => {
                    return hit_c;
                },
                None => {},
            }
        }
        // background color
        color(&self)
    }
}

fn color(r: &Ray) -> Rgb<u8> {
    let norm = r.dir.normalize();
    let t = 0.5 * (norm.y + 1.0);
    let v3 = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    // 0-1.0 f64 -> 0-255 u8
    let conv = |v: f64| -> u8 {
        (255.0 * v).clamp(0.0, 255.0) as u8
    };
    Rgb([conv(v3.x), conv(v3.y), conv(v3.z)])
}
