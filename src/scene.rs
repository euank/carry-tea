use image::RgbImage;
use image::Rgb;

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
                // Shift the camera from being in the upper left to being in the center
                // i.e. a camera at `0, 0, 0` width a world width of 5 should go from `(-2.5,
                // +2.5)` since we expect a camera to be the "center" of the viewport.
                let wx = wx - (self.camera.w / 2.0);
                let wy = wy - (self.camera.h / 2.0);

                let pix = Vec3::new(wx, wy, 0.0);
                let dir = (pix - self.camera.dir).normalize();
                let r = Ray::new(self.camera.pos, dir);
                let px = self.trace(&r);
                img.put_pixel(x, y, px);
            }
        }
        img
    }

    fn trace(&self, r: &Ray) -> Rgb<u8> {
        match self.closest_obj(r) {
            Some(inter) => {
                // attenuate the color based on the normal to show normals
                let objcolor = inter.obj.color();
                let n2 = 0.5 * (inter.norm + Vec3::new(1.0, 1.0, 1.0));
                return image::Rgb([
                        (objcolor[0] as f64 * n2.x) as u8,
                        (objcolor[1] as f64 * n2.y) as u8,
                        (objcolor[2] as f64 * n2.z) as u8,
                ])
            },
            None => {},
        }
        // background color
        color(r)
    }

    fn closest_obj(&self, r: &Ray) -> Option<RayIntersection> {
        self.objs
            .iter()
            .filter_map(|o| o.intersects(r))
            .min_by(|x, y| x.dist.partial_cmp(&y.dist).unwrap())
    }
}

pub(crate) struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
    pub w: f64,
    pub h: f64,
}

pub(crate) struct RayIntersection<'a> {
    // normal to the ray that hit this.
    norm: Vec3,
    // Distance from the ray
    dist: f64,
    // Point at which it intersected the object
    point: Vec3,
    // The object hit
    obj: &'a dyn Object,
}

// Objects are objects in our scene which may be hit by rays
pub(crate) trait Object {
    // Does this ray hit this object? If it does, at what distance does it intersect?
    fn intersects(&self, r: &Ray) -> Option<RayIntersection>;
    fn color(&self) -> image::Rgb<u8>;
}

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
