use crate::vec3::Vec3;
use crate::ray::Ray;


pub struct Camera {
    camera_position : Vec3,
    lower_left_corner : Vec3, 
    vertical_fov : Vec3,
    horizontal_fov : Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            camera_position : Vec3::zero(),
            lower_left_corner : Vec3::new( -2.0, -1.0, -1.0 ),
            vertical_fov : Vec3::new( 0.0, 2.0, 0.0 ),
            horizontal_fov : Vec3::new( 4.0, 0.0, 0.0 )
        }
    }

    pub fn get_ray(&self, u : f32, v : f32) -> Ray {
        Ray::new(self.camera_position, self.lower_left_corner + u * self.horizontal_fov + v * self.vertical_fov )
    }

}