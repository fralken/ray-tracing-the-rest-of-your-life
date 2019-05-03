use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::material::Material;
use crate::aabb::AABB;
use crate::onb::ONB;

fn get_sphere_uv(p: &Vector3<f32>) -> (f32, f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI);
    let v = (theta + f32::consts::FRAC_PI_2) / f32::consts::PI;
    (u, v)
}

fn random_to_sphere(radius: f32, distance_squared: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = 1.0 + r2 * ((1.0 - radius.powi(2) / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * f32::consts::PI * r1;
    let x = phi.cos() * (1.0 - z.powi(2)).sqrt();
    let y = phi.sin() * (1.0 - z.powi(2)).sqrt();
    Vector3::new(x, y, z)
}

#[derive(Clone)]
pub struct Sphere<M: Material> {
    center: Vector3<f32>,
    radius: f32,
    material: M
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector3<f32>, radius: f32, material: M) -> Self { Sphere {center, radius, material} }
}

impl<M: Material> Hitable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord { t, u, v, p, normal, material: &self.material })
            }
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                return Some(HitRecord { t, u, v, p, normal, material: &self.material })
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let radius = Vector3::new(self.radius, self.radius, self. radius);
        let min = self.center - radius;
        let max = self.center + radius;
        Some(AABB { min, max })
    }

    fn pdf_value(&self, o: Vector3<f32>, v: Vector3<f32>) -> f32 {
        if let Some(_hit) = self.hit(&Ray::new(o, v, 0.0), 0.001, f32::MAX) {
            let cos_theta_max = (1.0 - self.radius.powi(2) / (self.center - o).norm_squared()).sqrt();
            let solid_angle = 2.0 * f32::consts::PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, o: Vector3<f32>) -> Vector3<f32> {
        let direction = self.center - o;
        let distance_squared = direction.norm_squared();
        let uvw = ONB::build_from_w(&direction);
        uvw.local(&random_to_sphere(self.radius, distance_squared))
    }
}
