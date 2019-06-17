extern crate rand;

//std includes
use std::fs::File;
use std::io::{self, BufWriter, prelude::*};
use rand::Rng;
//local includes
mod ray;
pub mod vec3;
pub mod geometry;
pub mod scene;
pub mod material;

pub const F32_MAX : f32 = std::f32::MAX;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::scene::{world::*,camera::*};
use crate::geometry::HitRecord;
use crate::material::materials;

fn trace_ray(r: Ray, w: &World) -> Vec3 { 
    let rec = &mut HitRecord { t: 0., hit_point: Vec3::zero(), normal: Vec3::zero(), mat : Box::new(materials::Lambertian { albedo: Vec3::new(0.1,0.1,0.1)})} ;
    if w.hit(&r, 0.0001, F32_MAX, rec) {
        let mut ray_out = Ray::zero(); // will be overwritten in any case - do we really need to initialize ?
        let ray = &mut ray_out; // scatter will modify the ray_out -> need the &mut - but this doesnt look clean to me
        let attenuation = &mut Vec3::zero();
        if rec.mat.scatter(&r,rec, attenuation, ray) {
            let ray_out = ray_out;
            return *attenuation * trace_ray(ray_out,w);
        } else {
            return Vec3::new (0.0,0.0,0.0);
        }
    }
    let unit_direction = vec3::unit_vector(&r.direction);
    let t = 0.5 *( unit_direction.y + 1.0);
    vec3::lerp(&Vec3::one(), &Vec3::new(0.5, 0.7, 1.0), t)
}

fn create_image()-> io::Result<()> {
    let mut rng = rand::thread_rng();
    let nx = 200; 
    let ny = 100;
    let ray_per_pixel = 100;

    let world = World::new(2);
    let camera = Camera::new();
    let write_file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(&write_file);
    write!(&mut writer,"P3\n{} {}\n255\n",nx,ny)?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::zero();
            for _k in 0..ray_per_pixel {
                let u : f32 = (rng.gen::<f32>() + i as f32) / nx as f32;
                let v : f32 = (rng.gen::<f32>() + j as f32) / ny as f32;
                let r = camera.get_ray(u,v);
                color += trace_ray(r, &world); 
            }
            let color = color / ray_per_pixel as f32;
            let color = 255.99 * color.gamma_2_correct_simple_appx();
            write!(&mut writer,"{} {} {} \n", color.x as i32, color.y as i32, color.z as i32)?;
        }
        writeln!(&mut writer)?;  
    }
    Ok(())
}

fn main(){
    create_image();
}

