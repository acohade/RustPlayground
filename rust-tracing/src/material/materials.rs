use crate::ray::Ray; 
use crate::vec3::Vec3;
use crate::geometry::HitRecord;

use rand::Rng;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record : &HitRecord, attenuation : &mut Vec3, ray_out :&mut Ray ) -> bool;
    fn clone_boxed(&self) -> Box<Material>; // question to rustaceans - is there a better way of doing this ? Isnt there a Clone auto trait ? 
}

pub struct Lambertian {
   pub albedo : Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record : &HitRecord, attenuation : &mut Vec3, ray_out : &mut Ray ) -> bool {
        let random_bounce_location = hit_record.hit_point + hit_record.normal + Vec3::random_in_unit_sphere();
        ray_out.origin = hit_record.hit_point;
        ray_out.direction = random_bounce_location - hit_record.hit_point;
        *attenuation = self.albedo; // question to rustacean - is it good practive to use * to set a value ? 
        true
    }

    fn clone_boxed(&self) -> Box<Material>{
        Box::new(Lambertian {
            albedo : self.albedo
            })
     }
}

fn reflect( v : &Vec3, normal : & Vec3) -> Vec3{
    *v - 2. * Vec3::dot(v,normal) * *normal
}

fn schlick( cosine : f32, ref_idx : f32 ) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Metal {
    pub albedo : Vec3,
    pub roughness : f32
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record : &HitRecord, attenuation : &mut Vec3, ray_out : &mut Ray ) -> bool {
        let perfect_reflection = reflect(&ray_in.direction.unit_vector(), &hit_record.normal);
        let reflected = (Vec3::random() * self.roughness) + perfect_reflection;
        ray_out.origin = hit_record.hit_point;
        ray_out.direction = reflected;
        *attenuation = self.albedo; // question to rustacean - is it good practive to use * to set a value ? 
        Vec3::dot(&ray_out.direction,&hit_record.normal) > 0.
    }
    
    fn clone_boxed(&self) -> Box<Material>{
        Box::new(Metal { 
            albedo : self.albedo, 
            roughness: self.roughness
        })
     }
}

pub struct Dielectric {
    pub refraction_index : f32
}

impl Dielectric { 
    fn refract(&self, in_vec : Vec3, normal : Vec3, ni_over_nt :f32, refracted : &mut Vec3 ) -> bool {
        let uv = in_vec.unit_vector();
        let dt = Vec3::dot(&uv, &normal);
        let discriminant = 1.0 - ni_over_nt*ni_over_nt * (1.0-dt*dt);
        if discriminant > 0.0 {
            *refracted = ni_over_nt * (uv - normal*dt) - normal*discriminant.sqrt();
            return true;
        }
        false
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record : &HitRecord, attenuation : &mut Vec3, ray_out : &mut Ray ) -> bool {
        let refracted = &mut Vec3::zero();
        let reflected = reflect(&ray_in.direction.unit_vector(), &hit_record.normal);
        *attenuation = Vec3::one();

        //computing correct normals, refraction index, as well as reflection probability 
        let mut outward_normal = -hit_record.normal;
        let mut ni_over_nt = 0.0;;
        let mut cosine = 0.0;
        if Vec3::dot(&ray_in.direction, &hit_record.normal ) < 0.0 {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0/self.refraction_index;
            cosine = -Vec3::dot(&ray_in.direction, &
            
            
            hit_record.normal) / ray_in.direction.len();
        } else {
            ni_over_nt = self.refraction_index;
            cosine = self.refraction_index * Vec3::dot(&ray_in.direction, &hit_record.normal) / ray_in.direction.len();
        }

        ray_out.origin = hit_record.hit_point;
        let mut reflect_prob = 1.0;
        if self.refract(ray_in.direction, outward_normal, ni_over_nt, refracted) {
            reflect_prob = schlick(cosine, self.refraction_index);
        }
        let mut rng = rand::thread_rng();
        let random_val : f32 = rng.gen();   
        if  random_val < reflect_prob {
            ray_out.direction = reflected;
        } else {
            ray_out.direction = *refracted;
        }
        true
    }
    
    fn clone_boxed(&self) -> Box<Material>{
        Box::new(Dielectric { 
            refraction_index : self.refraction_index, 
        })
     }
}

