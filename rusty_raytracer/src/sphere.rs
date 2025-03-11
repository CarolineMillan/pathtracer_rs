use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};

pub struct Sphere {
    center: Point3<f32>,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Compute the ray-sphere intersection here.
        // This function would return Some(hit_record) if there's a valid intersection,
        // or None if the ray misses.
        // (Implementation based on the hit_sphere function from Chapter 6.)
        // ...

            // solving quadraatic equation for ray-sphere intersection
        // # roots = # intersections
        // change to return Option<f32>, so instead of -1 you can use None

        let oc = self.center - ray.origin();
        let a = ray.direction().norm_squared();      
        let h = ray.direction().dot(&oc);            // dot(direction, oc)
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None
        }
        
        let dis_sqrt = discriminant.sqrt();

        let mut root = (h-dis_sqrt)/a;

        if (root <= t_min) | (t_max <= root) {
            root = (h + dis_sqrt)/a;
            if (root <= t_min) | (t_max <= root) {
                return None
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p-self.center)/self.radius;

        //only returns one root...
        let mut rec = HitRecord::new_from(p, normal, t);

        rec.set_face_normal(ray, &normal);

        Some(rec)

        //Some((h - discriminant.sqrt()) / a)
        
}
}