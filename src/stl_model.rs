use anyhow::Result;
use image::Rgb;

use std::fs::OpenOptions;

use crate::scene::{Object, RayIntersection};
use crate::ray::Ray;
use crate::triangle::Triangle;

type Vec3 = crate::glm::TVec3<f64>;

pub struct StlModel {
    // TODO
    color: Rgb<u8>,
    triangles: Vec<Triangle>,
}

fn vert_to_vec(v: stl_io::Vertex) -> Vec3 {
    Vec3::new(v[0] as f64, v[1] as f64, v[2] as f64)
}

impl StlModel {
    pub fn new(path: &std::path::Path, color: Rgb<u8>) -> Result<Self> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let stl = stl_io::read_stl(&mut file)?;

        println!("loaded {} with {} triangles", path.to_string_lossy(), stl.faces.len());

        Ok(StlModel{
            color,
            triangles: stl.faces.iter().map(|t| {
                Triangle::new([
                    vert_to_vec(stl.vertices[t.vertices[0]]),
                    vert_to_vec(stl.vertices[t.vertices[1]]),
                    vert_to_vec(stl.vertices[t.vertices[2]]),
                ], color, true)
            }).collect(),
        })
    }
}

impl Object for StlModel {
    fn intersects(&self, r: &Ray) -> Option<RayIntersection> {
        self.triangles
            .iter()
            .filter_map(|o| o.intersects(r))
            .min_by(|x, y| x.dist.partial_cmp(&y.dist).unwrap())
    }

    fn color(&self) -> Rgb<u8> {
        self.color
    }
}
