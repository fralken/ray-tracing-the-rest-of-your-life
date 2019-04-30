use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::texture::Texture;

fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - unit;
        if p.magnitude_squared() < 1.0 {
            return p
        }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>, f32)>;

    fn scattering_pdf(&self, ray: &Ray, hit: &HitRecord, scattered: &Ray) -> f32;

    fn emitted(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32>;
}

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    albedo: T
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self { Lambertian { albedo } }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>, f32)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, (target - hit.p).normalize(), ray.time());
        let albedo = self.albedo.value(hit.u, hit.v, &hit.p);
        let pdf = hit.normal.dot(&scattered.direction()) / f32::consts::PI;
        Some((scattered, albedo, pdf))
    }

    fn scattering_pdf(&self, _ray: &Ray, hit: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = hit.normal.dot(&scattered.direction().normalize()).max(0.0);
        cosine / f32::consts::PI
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Vector3<f32>) -> Vector3<f32> { Vector3::zeros() }
}

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    emit: T
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self { DiffuseLight { emit } }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Vector3<f32>, f32)> { None }

    fn scattering_pdf(&self, _ray: &Ray, _hit: &HitRecord, _scattered: &Ray) -> f32 { 1.0 }

    fn emitted(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        self.emit.value(u, v, &p)
    }
}
