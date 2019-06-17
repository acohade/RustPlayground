use std:: { f32, 
            fmt };

use core::ops;
use rand::Rng;

// declaring vec
#[derive(Copy, Clone, Default)]
pub struct Vec3 { 
    pub x: f32,
    pub y: f32, 
    pub z: f32,
}

// formatting printing 
impl fmt::Display for Vec3{
    fn fmt (&self,f :&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {:.3}, {:.3}, {:.3} }}",self.x,self.y, self.z)
    }
}

pub fn dot(v1 : &Vec3, v: &Vec3) -> f32 {
        v1.x * v.x + 
        v1.y * v.y + 
        v1.z * v.z 
}

pub fn cross(v1 : &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {  x : v1.y * v2.z - v1.z * v2.y,
                y : v1.z * v2.x - v1.x * v2.z,
                z : v1.x * v2.y - v1.y * v2.x}
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
        let k = 1. / v.len();
        Vec3 {  x: v.x * k, 
                y: v.y * k, 
                z: v.z * k}
    }

pub fn lerp(v1 : &Vec3, v2: &Vec3, t: f32) -> Vec3 {
    (1.0 - t) * *v1 + t * *v2
}

impl Vec3{ 

    pub fn one() -> Vec3 {
        Vec3 
        { 
            x: 1.0,
            y: 1.0, 
            z: 1.0
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 
        { 
            x: 0.0,
            y: 0.0, 
            z: 0.0
        }
    }
    // random vector in unit cube
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen(),
            y: rng.gen(), 
            z: rng.gen()
        } * 2. - 1. 
    }

    // random vector in unit sphere 
    pub fn random_in_unit_sphere() -> Vec3 {
        // generate random in unit cube until it's in unit sphere
        let mut random_vector;
        loop {
            random_vector = Vec3::random();
            if random_vector.len() < 1. { break; }
        }
        random_vector
    }

    pub fn new(x_in: f32, y_in: f32, z_in: f32) -> Vec3 {
        Vec3
        {
            x:x_in,
            y:y_in, 
            z:z_in
        }
    }


    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn sq_len(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(&self) -> Vec3 {
        let len = self.len();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }    

    pub fn make_unit_vector(&mut self) {
        *self /= self.len();
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        dot(&self, &v)
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        cross(&self, &v)
    }

    pub fn gamma_2_correct_simple_appx(&self) -> Vec3 {
        Vec3
            {
                x : self.x.sqrt(),
                y : self.y.sqrt(), 
                z : self.z.sqrt()
            }
    }

}

//operator overloading 
// vector add 
impl ops::Add<Vec3> for Vec3{
    type Output = Vec3;
    fn add(self, val : Vec3) -> Vec3 {
        Vec3{   x : self.x + val.x, 
                y : self.y + val.y, 
                z : self.z + val.z}
    }
}
impl ops::AddAssign for Vec3{
    fn add_assign(& mut self, val : Vec3) {
       *self = Vec3 {   x : self.x + val.x, 
                        y : self.y + val.y, 
                        z : self.z + val.z}
    }
}

impl ops::Sub<Vec3> for Vec3{ 
    type Output = Vec3; 
    fn sub(self, val: Vec3) -> Vec3 { 
        Vec3 {  x: self.x - val.x, 
                y: self.y - val.y, 
                z: self.z - val.z}
    }
}
impl ops::SubAssign for Vec3{
    fn sub_assign(& mut self, val : Vec3) {
       *self = Vec3 {   x : self.x  - val.x, 
                        y : self.y -  val.y, 
                        z : self.z - val.z}
    }
}

impl ops::Mul<Vec3> for Vec3{ 
    type Output = Vec3; 
    fn mul(self, val: Vec3) -> Vec3 { 
        Vec3 {  x : self.x * val.x,
                y : self.y * val.y, 
                z : self.z * val.z}
    }
}
impl ops::MulAssign for Vec3{
    fn mul_assign(& mut self, val : Vec3) {
       *self = Vec3 {   x : self.x * val.x,
                        y : self.y * val.y,
                        z : self.z * val.z}
    }
}

impl ops::Div<Vec3> for Vec3{ 
    type Output = Vec3; 
    fn div(self, val: Vec3) -> Vec3 { 
        Vec3 {  x : self.x / val.x,
                y : self.y / val.y,
                z : self.z / val.z}
    }
}
impl ops::DivAssign for Vec3{
    fn div_assign(& mut self, val : Vec3) {
       *self = Vec3 {   x : self.x / val.x,
                        y : self.y / val.y,
                        z : self.z / val.z}
    }
}

impl ops::Add<f32> for Vec3{
    type Output = Vec3;
    fn add(self, val : f32) -> Vec3 {
        Vec3 {  x : self.x + val,
                y : self.y + val,
                z : self.z + val}
    }
}

impl ops::Sub<f32> for Vec3{
    type Output = Vec3;
    fn sub(self, val : f32) -> Vec3 {
        Vec3 {  x : self.x - val,
                y : self.y - val,
                z : self.z - val}
    }
}

impl ops::Mul<f32> for Vec3{
    type Output = Vec3;
    fn mul(self, val : f32) -> Vec3 {
        Vec3 {  x : self.x * val,
                y : self.y * val,
                z : self.z * val}
    }
}
impl ops::MulAssign<f32> for Vec3{
    fn mul_assign(& mut self, val : f32) {
       *self = Vec3 {   x : self.x * val,
                        y : self.y * val,
                        z : self.z * val}
    }
}

impl ops::Div<f32> for Vec3{
    type Output = Vec3;
    fn div(self, val : f32) -> Vec3 {
        Vec3 {  x : self.x / val,
                y : self.y / val,
                z : self.z / val}
    }
} 
impl ops::DivAssign<f32> for Vec3{
    fn div_assign(& mut self, val : f32) {
       *self = Vec3 {   x : self.x / val,
                        y : self.y / val,
                        z : self.z / val}
    }
}

impl ops::Mul<Vec3> for f32{
    type Output = Vec3;
    fn mul(self, val : Vec3) -> Vec3 {
        Vec3 {  x : self * val.x,
                y : self * val.y,
                z : self * val.z}
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


#[test]
fn it_implements_debug() {
    let vec = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 1.3,
    };
    let formatted_string = format!("{}", vec);
    let expected_string = "{ 0, 1, 1.3 }";
    assert_eq!(&formatted_string, expected_string);
}

