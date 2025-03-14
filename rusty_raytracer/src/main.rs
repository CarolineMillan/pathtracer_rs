
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
use nalgebra::{Point3, Vector3};
/* 
fn degrees_to_radians(degrees: f32) -> f32 {
    degrees*consts::PI / 180.0
}
    */

fn random_f32() -> f32 {
    let mut rng = rng();
    rng.random()
}

fn random_f32_within(min: f32, max: f32) -> f32 {
    let mut rng = rng();
    let random_f32: f32 = rng.random();
    min + (max-min)*random_f32
}

fn random_vec3() -> Vector3<f32> {

    let x = random_f32();
    let y = random_f32();
    let z = random_f32();
    let random_vec = Vector3::new(x,y,z);
    random_vec
}

fn random_vec3_within(min: f32, max: f32) -> Vector3<f32> {
    // there's got to be a better way to do this with nalgebra
    let x = random_f32_within(min, max);
    let y = random_f32_within(min, max);
    let z = random_f32_within(min, max);

    let random_vec = Vector3::new(x,y,z);
    random_vec
}

fn random_unit_vector() -> Option<Vector3<f32>> {
    // not sure abt 1e-160 bit
    loop {
        let p = random_vec3_within(-1.0, 1.0);
        let lensq = p.norm_squared();
        if (1e-8 < lensq) && (lensq <= 1.0) {return Some(p/(lensq.sqrt()))}
    }
}

fn random_on_hemisphere(normal: &Vector3<f32>) -> Vector3<f32> {
    let on_unit_sphere = random_unit_vector().expect("No random unit vector!");
    if normal.dot(&on_unit_sphere) > 0.0 {return on_unit_sphere} else {return -on_unit_sphere}
}

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
    cam.max_depth = 50;

    let _ = cam.render(&world);

    Ok(())
}