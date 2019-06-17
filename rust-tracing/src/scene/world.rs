use std::vec::Vec;

use crate::geometry::{
    HitRecord,
    sphere::Sphere
};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::materials;

pub struct World {
    objects: Vec<Sphere>
}

impl World {
    pub fn new(_list_size: i32) -> World {
        let list : Vec<Sphere> = vec! (
            Sphere::new( &Vec3::new( 0.0, 0.0, -1.0 )   , 0.5   , Box::new(materials::Lambertian    { albedo: Vec3::new( 0.8, 0.3, 0.3 )})),
            Sphere::new( &Vec3::new( 0.0, -100.5, -1.0) , 100.  , Box::new(materials::Lambertian    { albedo: Vec3::new( 0.8, 0.8, 0.0 )})),
            Sphere::new( &Vec3::new( 1.0, 0.0, -1.0 )   , 0.5   , Box::new(materials::Metal         { albedo: Vec3::new( 0.8, 0.6, 0.2 )})),
            Sphere::new( &Vec3::new( -1.0, -0.2, -1.0 ) , 0.3   , Box::new(materials::Metal        { albedo: Vec3::new( 0.8, 0.8, 0.8 )}))
         );
        World { objects : list }
    }

    pub fn hit(&self, r : &Ray, t_min : f32, t_max : f32 ,  rec : &mut HitRecord) -> bool{
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for element in self.objects.iter(){
            if element.hit(r, t_min, closest_so_far, rec) { 
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }
}