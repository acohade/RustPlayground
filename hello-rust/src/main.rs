use std::fs::File;
use std::io::{self, BufWriter, prelude::*};

fn main() -> io::Result<()> {
    let nx = 200; 
    let ny = 100;
    let write_file = File::create("foo.ppm")?;
    let mut writer = BufWriter::new(&write_file);
    write!(&mut writer,"P3\n{} {}\n255\n",nx,ny)?;

    for i in 0..ny {
        for j in 0..nx {
            let g = (ny - i) as f32 / ny as f32;
            let r = j as f32 / nx as f32;
            let ir = 255.99 * r;
            let ig = 255.99 * g;
            let ib = 255.99 * 0.2;
            write!(&mut writer,"{} {} {} \n", ir as i32, ig as i32, ib as i32)?;
        }
        writeln!(&mut writer)?;  
    }
    Ok(())
}