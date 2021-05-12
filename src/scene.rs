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
        if b * b - 4.0*a*c > 0.0 {
            Some(self.color)
        } else {
            None
        }
    }
}
