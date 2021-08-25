use nalgebra_glm as glm;

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
}
