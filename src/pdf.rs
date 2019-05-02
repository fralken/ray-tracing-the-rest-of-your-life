use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use crate::onb::ONB;

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
    fn value(&self, direction: &Vector3<f32>) -> f32;
    fn generate(&self) -> Vector3<f32>;
}

pub struct CosinePDF {
    uvw: ONB
}

impl CosinePDF {
    pub fn new(w: &Vector3<f32>) -> Self {
        CosinePDF { uvw: ONB::build_from_w(&w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vector3<f32>) -> f32 {
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