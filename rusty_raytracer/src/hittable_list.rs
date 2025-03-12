use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
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
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // returns a HitRecord if ray intersects an object between t_min and t_max
        
        let mut closest_so_far  = ray_t.max;
        let mut final_hit = None;
        let mut count = 1;

        for object in self.objects.iter() {
            //println!("in hittable list: {}", count);
            if let Some(hit_record) = object.hit(ray, &Interval::new(ray_t.min, closest_so_far)) {
                // we never get here
                //println!("in if");
                closest_so_far = hit_record.t;
                final_hit = Some(hit_record);
            }
            count +=1;
        }
        //println!("final hit? {}", final_hit.is_some());
        final_hit
    }
}