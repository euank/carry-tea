use crate::ray::Ray;

type Vec3 = crate::glm::TVec3<f64>;

// A Scene contains objects
pub(crate) struct Scene {
    // TODO: camera
    pub(crate) objs: Vec<Box<dyn Object>>,
}

// Objects are objects in our scene which may be hit by rays
pub(crate) trait Object {
    // Does this ray hit this object? If it does, what color does it produce?
    fn is_hit(&self, r: &Ray) -> Option<image::Rgb<u8>>;
}

pub(crate) struct Sphere {
    pub(crate) center: Vec3,
    pub(crate) radius: f64,
    pub(crate) color: image::Rgb<u8>,
}

impl Object for Sphere {
    fn is_hit(&self, r: &Ray) -> Option<image::Rgb<u8>> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let oc = r.origin - self.center;
        let a = r.dir.dot(&r.dir);
        let b = 2.0 * oc.dot(&r.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let disc = b * b - 4.0*a*c;
        if disc < 0.0 {
            return None;
        }
        let t = (-b - disc.sqrt()) / (2.0 * a);
        let norm: Vec3 = (r.trace(t) - self.center).normalize();
        let n2 = 0.5 * (norm + Vec3::new(1.0, 1.0, 1.0));
        // lighter color the worse the normal just to see normals
        return Some(image::Rgb([
            (self.color[0] as f64 * n2.x) as u8,
            (self.color[1] as f64 * n2.y) as u8,
            (self.color[2] as f64 * n2.z) as u8,
        ]))
    }
}
