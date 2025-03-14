use crate::{colour::Colour, hittable::HitRecord, ray::Ray};


pub trait Material {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        None
    }
    fn clone_box(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}