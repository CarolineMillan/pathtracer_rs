// responsible for:
// - constructing and dispatching rays into the world
// - using the results of these rays to construct the rendered image

use std::{fmt::Result, io::Write};
use std::fs::File;
use std::io;
//use std::io::Error;
use nalgebra::{Point3, Vector3};

use crate::interval::Interval;
use crate::{hittable::Hittable, hittable_list::HittableList, ray::Ray, colour::{write_colour, Colour}};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: f32,
    image_height: f32,
    center: Point3<f32>,
    pixel00_loc: Point3<f32>,
    pixel_delta_u: Vector3<f32>,
    pixel_delta_v: Vector3<f32>,

}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 0.0,
            image_width: 0.0,
            image_height: 0.0,
            center: Point3::origin(),
            pixel00_loc: Point3::origin(),
            pixel_delta_u: Vector3::zeros(),
            pixel_delta_v: Vector3::zeros(),
        }
    }
    pub fn initialise(&mut self) {
        // Ensure dimensions are correctly set
        self.image_height = self.image_width / self.aspect_ratio;
        if self.image_height < 1.0 {
            self.image_height = 1.0;
        }
    
        self.center = Point3::new(0.0, 0.0, 0.0);
    
        // Camera setup
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width / self.image_height);
    
        // Ensure viewport sizes are sensible
        println!("Viewport width: {}", viewport_width);
        println!("Viewport height: {}", viewport_height);
    
        // Vectors along viewport edges
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
    
        // Pixel deltas
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;
    
        // Upper-left pixel location
        let viewport_upper_left = self.center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
    
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    
        // Double-check deltas
        println!("Pixel deltas: u = {:?}, v = {:?}", self.pixel_delta_u, self.pixel_delta_v);
    }
    
/* 
    pub fn initialise(&mut self) {
        // image dimensions
        self.image_height = self.image_width/self.aspect_ratio;
        if self.image_height < 1.0 {self.image_height = 1.0}

        self.center = Point3::new(0.0, 0.0, 0.0);

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width/self.image_height);

        // vectors along viewport edges
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        //how much of the vector is equal to one pixel?
        let pixel_delta_u = viewport_u / self.image_width;
        let pixel_delta_v = viewport_v / self.image_height;

        //locate upper left pixel
        let viewport_upper_left = 
            self.center - Vector3::new(0.0,0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    }
    */

    pub fn render(&mut self, world: &HittableList) -> io::Result<()> {

        self.initialise();
        // render
        let mut file = File::create("rendered_image.ppm")?;
    
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
    
        file.write_all(header.as_bytes())?;
    
        for j in 0..self.image_height as usize {
            println!("\rScanlines remaining: {} ", (self.image_height as usize)-j);
            for i in 0..self.image_width as usize {

                let pixel_center = self.pixel00_loc + (i as f32* self.pixel_delta_u) + ((j as f32)* self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new_from(self.center, ray_direction);
    
                //println!("Ray origin: {:?}", r.origin());
                //println!("Ray direction: {:?}", r.direction());


                let pixel_colour = ray_colour(&r, &world);
    
                let _res = write_colour(&file, pixel_colour);
            }
        }
        println!("\rDone.               \n");
        Ok(())
    }
}

fn ray_colour(ray: &Ray, world: &HittableList) -> Colour {
    let potential_hit = world.hit(ray, &Interval::new(0.001, f32::INFINITY));
    //println!("in ray colour: {}", potential_hit.is_some());
    if potential_hit.is_some() {
        // we never get here
        //println!("in ray colour hit");
        let mapped = 0.5*(potential_hit.unwrap().normal + Vector3::new(1.0, 1.0, 1.0));
        return Colour::new_from(mapped.x, mapped.y, mapped.z);
    }

    // else draw sky
    let unit_direction = ray.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    let ans = (1.0-a)*Colour::new_from(1.0, 1.0, 1.0).0 + a*Colour::new_from(0.5, 0.7, 1.0).0;
    Colour::new_from(ans[0], ans[1], ans[2])
}