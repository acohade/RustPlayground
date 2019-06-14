pub mod sphere;

use crate::vec3::Vec3;

pub struct HitRecord {
    pub t : f32, 
    pub hit_point : Vec3, 
    pub normal : Vec3
}