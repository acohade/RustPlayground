use std::fs::File;
use std::io::{self, BufWriter, prelude::*};


fn my_func (test : &mut f32){
    *test = 5.;

}
fn main() {

    let mut my_val : f32 = 4.;
    println!("before = {}", my_val );
    my_func(&mut my_val);
    println!("after = {}", my_val );


}