use image::Rgb;

use crate::scene::{Object, RayIntersection};
use crate::ray::Ray;

type Vec3 = crate::glm::TVec3<f64>;

pub(crate) struct Triangle {
    pub(crate) norm: Vec3,
    pub(crate) verts: [Vec3; 3],
    pub(crate) color: image::Rgb<u8>,
}

impl Triangle {
    pub fn new(verts: [Vec3; 3], color: image::Rgb<u8>, heads: bool) -> Self {
        let norm = if heads {
            (verts[1] - verts[0]).cross(&(verts[2] - verts[0]))
        } else {
            (verts[2] - verts[0]).cross(&(verts[1] - verts[0]))
        };
        Triangle{
            verts,
            norm,
            color,
        }
    }
}

impl Object for Triangle {
    fn intersects(&self, r: &Ray) -> Option<RayIntersection> {
        // https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm#C++_implementation
        let edge1 = self.verts[1] - self.verts[0];
        let edge2 = self.verts[2] - self.verts[0];
        let h = r.dir.cross(&edge2);
        let a = edge1.dot(&h);
        if a < 0.0001 && a > -0.0001 {
            return None;
        }
        let f = 1.0 / a;
        let s = r.origin - self.verts[0];
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(&edge1);
        let v = f * r.dir.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = f * edge2.dot(&q);
        if t > 0.0001 {
            Some(RayIntersection{
                norm: self.norm,
                obj: self,
                point: r.origin + r.dir * t,
                dist: t,
            })
        } else {
            None
        }
    }

    fn color(&self) -> Rgb<u8> {
        self.color
    }
}
