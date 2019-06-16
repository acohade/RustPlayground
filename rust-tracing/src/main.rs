//std includes
use std::fs::File;
use std::io::{self, BufWriter, prelude::*};
//local includes
mod ray;
pub mod vec3;
pub mod geometry;
pub mod scene;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::geometry::HitRecord;



fn color(r: Ray, s: &Scene) -> Vec3 { 
    let mut rec = HitRecord { t: 0., hit_point: Vec3::zero(), normal: Vec3::zero()} ;
    let rec = &mut rec;
    if s.hit(&r, 0.0, 9999999., rec) {
        return 0.5 * (rec.normal + 1.);
    }
    let unit_direction = vec3::unit_vector(&r.direction);
    let t = 0.5 *( unit_direction.y + 1.0);
    vec3::lerp(&Vec3::one(), &Vec3::new(0.5, 0.7, 1.0), t)
}

fn create_image()-> io::Result<()> {

    let nx = 200; 
    let ny = 100;
    let camera_position = Vec3::zero();
    let lower_left_corner = Vec3::new( -2.0, -1.0, -1.0 );
    let vertical = Vec3::new( 0.0, 2.0, 0.0 );
    let horizontal = Vec3::new( 4.0, 0.0, 0.0 );
    let world = Scene::new(2);

    let write_file = File::create("image.ppm")?;
    let mut writer = BufWriter::new(&write_file);
    write!(&mut writer,"P3\n{} {}\n255\n",nx,ny)?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u : f32 = i as f32 / nx as f32;
            let v : f32 = j as f32 / ny as f32;
            let r : Ray = Ray::new(camera_position, lower_left_corner + u * horizontal + v * vertical);
            let color = color(r, &world);
            let color = 255.99 * color;
            write!(&mut writer,"{} {} {} \n", color.x as i32, color.y as i32, color.z as i32)?;
        }
        writeln!(&mut writer)?;  
    }
    Ok(())
}

fn main(){
    create_image();
}

