use crate::vector::vector::{Point3d, Vector3d};

pub struct Ray {
    pub origin: Point3d,
    pub direction: Vector3d,
}

impl Ray {
    pub fn new(origin: Point3d, direction: Vector3d) -> Ray {
        Self { origin, direction }
    }
    
    pub fn at(&self, t: f64) -> Point3d {
        self.origin + self.direction * t
    }
}
