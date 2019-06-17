pub mod sphere;

use crate::vec3::Vec3;
use crate::material::materials;

// todo - this doesnt belong here - should be part of a separate ray-tracing module, with Ray 
pub struct HitRecord {
    pub t : f32, 
    pub hit_point : Vec3, 
    pub normal : Vec3,
    pub mat : Box<materials::Material>
}