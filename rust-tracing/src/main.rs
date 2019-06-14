//std includes
use std::fs::File;
use std::io::{self, BufWriter, prelude::*};
//local includes
mod vec3;

fn create_image()-> io::Result<()> {
    let nx = 200; 
    let ny = 100;
    let write_file = File::create("foo.ppm")?;
    let mut writer = BufWriter::new(&write_file);
    write!(&mut writer,"P3\n{} {}\n255\n",nx,ny)?;

    for i in (0..ny).rev() {
        for j in 0..nx {
            let color = vec3::Vec3 { 
                x: j as f32 / nx as f32,
                y: i as f32 / ny as f32, 
                z: 0.2 };
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

