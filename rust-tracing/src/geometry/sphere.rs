use crate::vec3::*;
use crate::ray::Ray;
use crate::geometry::HitRecord;


pub struct Sphere {
    pub radius : f32, 
    pub center : Vec3
}

impl Sphere {
    pub fn new(c: &Vec3, r: f32) -> Sphere {
        Sphere { radius: r, center: *c}
    }



    pub fn hit(&self,r: &Ray, t_min : f32, t_max : f32, hit_record : &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = dot(&r.direction, &r.direction);
        let b = dot(&oc, &r.direction);
        let c = dot(&oc,&oc) - self.radius * self.radius;
        // root for quadratic equation solution
        // x = [-b +- sqrt(b * b - 4 * a * c)] / 2 * a
        // b2 - 4ac is called discriminant
        // if b2-4ac < 0 sqrt is not real -> no real solution 
        // if = 0, sqrt = 0 -> one solution 
        // if > 0, both + and - are solutions -> 2 solutions
        // http://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf
        let discriminant = b * b - a* c;
        // not hiting sphere
        if discriminant > 0. {  
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.hit_point = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.hit_point - self.center)/ self.radius;
                return true;
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.hit_point = r.point_at_parameter(hit_record.t);
                hit_record.normal = (hit_record.hit_point - self.center) / self.radius;
                return true;
            }
        }
        false
    }

}