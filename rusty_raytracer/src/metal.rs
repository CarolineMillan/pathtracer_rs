use crate::{colour::Colour, hittable::HitRecord, material::Material, near_zero, random_unit_vector, ray::Ray, reflect};


pub struct Metal {
    albedo: Colour,
}

impl Metal {
    pub fn new() -> Self {
        Self {
            albedo: Colour::new(),
        }
    }

    pub fn new_from(albedo: Colour) -> Self {
        Self {
            albedo,
        }
    }
    
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        
        let reflected = reflect(&r_in.direction(), &rec.normal);

        let scattered = Ray::new_from(rec.p, reflected);
        let attenuation = self.albedo.clone();
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Metal {
    fn clone(&self) -> Self {
        Self {
            albedo: self.albedo.clone(),
        }
    }
}