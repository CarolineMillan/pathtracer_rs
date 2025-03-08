
mod colour;

use std::fs::File;
use std::io::Write;
use colour::{Colour, write_colour};

// no need to write your own Vector3
//use nalgebra::Vector3;

pub fn main() -> std::io::Result<()>{

    let image_width = 256;
    let image_height = 256;
    let mut file = File::create("rendered_image.ppm")?;

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);

    file.write_all(header.as_bytes())?;

    for j in 0..image_height {
        println!("\rScanlines remaining: {} ", image_height-j);
        for i in 0..image_width {
            let r = (i as f32)/(image_width-1)as f32;
            let g = (j as f32)/(image_height-1) as f32;
            let b = 0.000;

            let pixel_colour = Colour::new_from(r,g,b);
            let _res = write_colour(&file, pixel_colour);
        }
    }
    println!("\rDone.               \n");
    Ok(())
}