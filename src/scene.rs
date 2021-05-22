use image::RgbImage;

use crate::ray::Ray;

type Vec3 = crate::glm::TVec3<f64>;

// A Scene contains objects
pub(crate) struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub(crate) objs: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn render(&self) -> image::RgbImage {
        let width = self.width;
        let height = self.height;

        let mut img = RgbImage::new(width, height);
        for y in (0..height).rev() {
            for x in 0..width {
                let wx = x as f64 / (width-1) as f64 * self.camera.w;
                let wy = y as f64 / (height-1) as f64 * self.camera.h;

                let pix = Vec3::new(wx, wy, 0.0);
                let dir = (pix - self.camera.dir).normalize();
                let r = Ray::new(self.camera.pos, dir);

                let px = r.trace(&self);
                img.put_pixel(x, y, px);
            }
        }
        img
    }
}

pub(crate) struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
    pub w: f64,
    pub h: f64,
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
        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let t = if t0 < t1 && t0 > 0.00001 {
            t0
        } else {
            t1
        };
        let hit = r.origin + t * r.dir;

        let norm = 1.0 / self.radius * (hit - self.center);
        let n2 = 0.5 * (norm + Vec3::new(1.0, 1.0, 1.0));
        // lighter color the worse the normal just to see normals
        return Some(image::Rgb([
            (self.color[0] as f64 * n2.x) as u8,
            (self.color[1] as f64 * n2.y) as u8,
            (self.color[2] as f64 * n2.z) as u8,
        ]))
    }
}
