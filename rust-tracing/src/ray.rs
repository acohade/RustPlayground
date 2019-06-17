use crate::vec3::*;


pub struct Ray { 
    pub origin: Vec3, 
    pub direction: Vec3,
}

impl Ray { 
    pub fn new(v1 : Vec3, v2 : Vec3,) -> Ray {
        Ray{ origin: v1, direction: v2}
    }

    pub fn zero() -> Ray {
        Ray{ origin: Vec3::zero(), direction: Vec3::zero()}
    }

    pub fn point_at_parameter(&self, t : f32) -> Vec3 {
        self.origin + t * self.direction
    }

}