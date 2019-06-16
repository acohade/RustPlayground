use std::vec::Vec;
use crate::geometry::{
    HitRecord,
    sphere::Sphere
};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct World {
    objects: Vec<Sphere>
}

impl World {
    pub fn new(list_size: i32) -> World {
        let mut list : Vec<Sphere> = Vec::with_capacity(2);
        list.push(Sphere::new( &Vec3::new( 0., 0., -1. ), 0.5));
        list.push(Sphere::new( &Vec3::new( 0., -100.5, -1. ), 100.));
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