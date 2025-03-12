
mod colour;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;

use std::fs::File;
use std::io::Write;
use std::f32::{self, INFINITY};
use std::f32::consts;
use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use interval::Interval;

use crate::colour::{Colour, write_colour};
use crate::ray::Ray;

// no need to write your own Vector3
use nalgebra::{Point3, Vector3};

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees*consts::PI / 180.0
}

fn ray_colour(ray: &Ray, world: &HittableList) -> Colour {
    let potential_hit = world.hit(ray, &Interval::new(0.001, f32::INFINITY));

    if potential_hit.is_some() {
        let mapped = 0.5*(potential_hit.unwrap().normal + Vector3::new(1.0, 1.0, 1.0));
        return Colour::new_from(mapped.x, mapped.y, mapped.z);
    }

    // else draw sky
    let unit_direction = ray.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    let ans = (1.0-a)*Colour::new_from(1.0, 1.0, 1.0).0 + a*Colour::new_from(0.5, 0.7, 1.0).0;
    Colour::new_from(ans[0], ans[1], ans[2])
}

pub fn main() -> std::io::Result<()>{

    // image dimensions
    let aspect_ratio = 16.0/9.0;
    let image_width = 400.0;
    let mut image_height = image_width/aspect_ratio;
    if image_height < 1.0 {image_height = 1.0}

    //World

    let mut world = HittableList::new();

    let sphere1 = Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    let sphere2 = Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    //tutorial uses "make_shared" here FIXME
    world.add(sphere1);
    world.add(sphere2);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width/image_height);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // vectors along viewport edges
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    //how much of the vector is equal to one pixel?
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    //locate upper left pixel
    let viewport_upper_left = camera_center - Vector3::new(0.0,0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // render
    let mut file = File::create("rendered_image.ppm")?;

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);

    file.write_all(header.as_bytes())?;

    for j in 0..image_height as usize {
        println!("\rScanlines remaining: {} ", (image_height as usize)-j);
        for i in 0..image_width as usize {

            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32* pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new_from(camera_center, ray_direction);

            let pixel_colour = ray_colour(&r, &world);

            let _res = write_colour(&file, pixel_colour);
        }
    }
    println!("\rDone.               \n");
    Ok(())
}