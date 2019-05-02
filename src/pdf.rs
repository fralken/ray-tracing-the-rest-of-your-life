use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use crate::onb::ONB;
use crate::hitable::Hitable;

pub fn random_cosine_direction() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * f32::consts::PI * r1;
    let x = phi.cos() * 2.0 * r2.sqrt();
    let y = phi.sin() * 2.0 * r2.sqrt();
    Vector3::new(x, y, z)
}

pub trait PDF {
    fn value(&self, direction: Vector3<f32>) -> f32;
    fn generate(&self) -> Vector3<f32>;
}

pub struct CosinePDF {
    uvw: ONB
}

impl CosinePDF {
    pub fn new(w: Vector3<f32>) -> Self {
        CosinePDF { uvw: ONB::build_from_w(&w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vector3<f32>) -> f32 {
        let cosine = direction.normalize().dot(&self.uvw.w());
        if cosine > 0.0 {
            cosine / f32::consts::PI
        } else {
            0.0
        }
    }

    fn generate(&self) -> Vector3<f32> {
        self.uvw.local(&random_cosine_direction())
    }
}

pub struct HitablePDF<'a> {
    origin: Vector3<f32>,
    hitable: &'a Hitable
}

impl<'a> HitablePDF<'a> {
    pub fn new(hitable: &'a Hitable, origin: Vector3<f32>) -> Self {
        HitablePDF { origin, hitable }
    }
}

impl<'a> PDF for HitablePDF<'a> {
    fn value(&self, direction: Vector3<f32>) -> f32 {
        self.hitable.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vector3<f32> { self.hitable.random(self.origin) }
}

pub struct MixturePDF<P: PDF, Q: PDF> {
    p: P,
    q: Q
}

impl<P: PDF, Q: PDF> MixturePDF<P, Q> {
    pub fn new(p: P, q: Q) -> Self { MixturePDF { p, q } }
}

impl<P: PDF, Q: PDF> PDF for MixturePDF<P, Q> {
    fn value(&self, direction: Vector3<f32>) -> f32 {
        0.5 * self.p.value(direction) + 0.5 * self.q.value(direction)
    }

    fn generate(&self) -> Vector3<f32> {
        let mut rng = rand::thread_rng();
        if rng.gen::<bool>() {
            self.p.generate()
        } else {
            self.q.generate()
        }
    }
}