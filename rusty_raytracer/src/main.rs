
mod colour;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;

use std::f32;
use rand::{rng, Rng}; //random number generator

use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;

// no need to write your own Vector3
use nalgebra::Point3;
/* 
fn degrees_to_radians(degrees: f32) -> f32 {
    degrees*consts::PI / 180.0
}
    */

fn random_f32() -> f32 {
    let mut rng = rng();
    rng.random()
}
/*
fn random_f32_within(min: f32, max: f32) -> f32 {
    let mut rng = rng();
    let random_f32: f32 = rng.random();
    min + (max-min)*random_f32
}
*/
pub fn main() -> std::io::Result<()>{

    //World
    let mut world = HittableList::new();

    let sphere1 = Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let sphere2 = Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    //tutorial uses "make_shared" here FIXME
    world.add(sphere1);
    world.add(sphere2);

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400.0;
    cam.samples_per_pixel = 100;

    let _ = cam.render(&world);

    Ok(())
}