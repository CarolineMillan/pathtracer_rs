
mod colour;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;

use std::fs::File;
use std::io::Write;
use std::f32::{self, INFINITY};
use std::f32::consts;
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use interval::Interval;
use camera::Camera;


use crate::colour::{Colour, write_colour};
use crate::ray::Ray;

// no need to write your own Vector3
use nalgebra::{Point3, Vector3};

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees*consts::PI / 180.0
}



pub fn main() -> std::io::Result<()>{

    //World

    let mut world = HittableList::new();

    let sphere1 = Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let sphere2 = Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    //tutorial uses "make_shared" here FIXME
    world.add(sphere1);
    world.add(sphere2);

    //println!("world: {}", world.objects);

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400.0;

    let _ = cam.render(&world);

    Ok(())
}