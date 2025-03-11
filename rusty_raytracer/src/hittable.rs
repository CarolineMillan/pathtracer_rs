use crate::ray::Ray;

use nalgebra::{Point3, Vector3};

pub trait Hittable {
    // a trait will be used as a sort of "parent class" for hittable objects
    
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        None
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    front_face: bool,
}

impl HitRecord {

    // rename to default?
    pub fn new() -> Self {
        Self {
            p: Point3::origin(),
            normal: Vector3::zeros(),
            t: 0.0,
            front_face: false, //FIXME
        }
    }

    pub fn new_from(p: Point3<f32>, normal: Vector3<f32>, t: f32) -> Self {
        Self {
            p,
            normal,
            t,
            front_face: false, // FIXME
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3<f32>) {
        // set normal vector

        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if front_face {outward_normal.clone()} 
        else {-outward_normal};

    }
}