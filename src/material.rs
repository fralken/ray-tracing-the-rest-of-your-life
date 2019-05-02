use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::texture::Texture;
use crate::onb::ONB;

fn random_cosine_direction() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * f32::consts::PI * r1;
    let x = phi.cos() * 2.0 * r2.sqrt();
    let y = phi.sin() * 2.0 * r2.sqrt();
    Vector3::new(x, y, z)
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
        let uvw = ONB::build_from_w(&hit.normal);
        let direction = uvw.local(&random_cosine_direction());
        let scattered = Ray::new(hit.p, direction.normalize(),ray.time());
        let albedo = self.albedo.value(hit.u, hit.v, &hit.p);
        let pdf = uvw.w().dot(&scattered.direction()) / f32::consts::PI;
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
