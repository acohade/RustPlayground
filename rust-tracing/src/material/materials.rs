use crate::ray::Ray; 
use crate::vec3::Vec3;
use crate::geometry::HitRecord;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record : &HitRecord, attenuation : &mut Vec3, ray_out :&mut Ray ) -> bool;
    fn clone_boxed(&self) -> Box<Material>;
}

pub struct Lambertian {
   pub albedo : Vec3

}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record : &HitRecord, attenuation : &mut Vec3, ray_out : &mut Ray ) -> bool {
        let bounce_vector = hit_record.hit_point + hit_record.normal + Vec3::random_in_unit_sphere();
        ray_out.origin = hit_record.hit_point;
        ray_out.direction = bounce_vector - hit_record.hit_point;
        *attenuation = self.albedo;
        true
    }
     fn clone_boxed(&self) -> Box<Material>{
        Box::new(Lambertian {albedo : self.albedo})
     }
}
fn reflect( v : &Vec3, normal : & Vec3) -> Vec3{
    *v - 2. * Vec3::dot(v,normal) * *normal
}

pub struct Metal {
    pub albedo : Vec3
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record : &HitRecord, attenuation : &mut Vec3, ray_out : &mut Ray ) -> bool {
        let reflected = reflect(&ray_in.direction.unit_vector(), &hit_record.normal); 
        ray_out.origin = hit_record.hit_point;
        ray_out.direction = reflected;
        *attenuation = self.albedo;
        Vec3::dot(&ray_out.direction,&hit_record.normal) > 0.
    }
    fn clone_boxed(&self) -> Box<Material>{
        Box::new(Metal {albedo : self.albedo})
     }
}