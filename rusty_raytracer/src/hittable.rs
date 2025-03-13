use crate::ray::Ray;
use crate::interval::Interval;

use nalgebra::{Point3, Vector3};

pub trait Hittable {
    // a trait will be used as a sort of "parent class" for hittable objects
    
    fn hit(&self, _ray: &Ray, _ray_t: &Interval) -> Option<HitRecord> {
        None
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub _p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    _front_face: bool,
}

impl HitRecord {

    // rename to default?
    /*
    pub fn new() -> Self {
        Self {
            _p: Point3::origin(),
            normal: Vector3::zeros(),
            t: 0.0,
            _front_face: false, //FIXME
        }
    }
    */

    pub fn new_from(p: Point3<f32>, normal: Vector3<f32>, t: f32) -> Self {
        Self {
            _p: p,
            normal,
            t,
            _front_face: false, // FIXME
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3<f32>) {
        // set normal vector

        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if front_face {outward_normal.clone()} 
        else {-outward_normal};

    }
}