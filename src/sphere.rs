use image::Rgb;

use crate::scene::{Object, RayIntersection};
use crate::ray::Ray;

type Vec3 = crate::glm::TVec3<f64>;

pub(crate) struct Sphere {
    pub(crate) center: Vec3,
    pub(crate) radius: f64,
    pub(crate) color: image::Rgb<u8>,
}

impl Object for Sphere {
    fn intersects(&self, r: &Ray) -> Option<RayIntersection> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let oc = r.origin - self.center;
        let a = r.dir.dot(&r.dir);
        let b = 2.0 * oc.dot(&r.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let disc = b * b - 4.0*a*c;
        if disc < 0.0 {
            return None;
        }
        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let t = if t0 < t1 && t0 > 0.00001 {
            t0
        } else {
            t1
        };
        let hit = r.origin + t * r.dir;

        let norm = 1.0 / self.radius * (hit - self.center);
        Some(RayIntersection{
            dist: t.abs(),
            norm,
            point: hit,
            obj: self,
        })
    }

    fn color(&self) -> Rgb<u8> {
        self.color
    }
}
