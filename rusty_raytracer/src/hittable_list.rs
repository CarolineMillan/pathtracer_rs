use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct HittableList {
    // Read more about Boxes, and shared pointers in cpp
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {

    pub fn new() -> Self {
        // creates a new empty list of objects
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        // adds an object to objects
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        // clears the list of objects
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // returns a HitRecord if ray intersects an object between t_min and t_max
        
        let mut closest_so_far  = t_max;
        let mut potential_hit = None;

        for object in &self.objects {
            potential_hit = object.hit(ray, t_min, closest_so_far);
            if potential_hit.is_some() {
                closest_so_far = potential_hit.clone().unwrap().t;
            }
        }
        potential_hit
    }
}