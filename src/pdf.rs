use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use crate::onb::ONB;
use crate::hitable::Hitable;

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

pub enum PDF<'a> {
    Cosine { uvw: ONB },
    Hitable { origin: Vector3<f32>, hitable: &'a Box<dyn Hitable> },
    Mixture { p: &'a PDF<'a>, q: &'a PDF<'a> }
}

impl<'a> PDF<'a> {
    pub fn cosine(w: Vector3<f32>) -> Self {
        PDF::Cosine { uvw: ONB::build_from_w(&w) }
    }

    pub fn hitable(hitable: &'a Box<dyn Hitable>, origin: Vector3<f32>) -> Self {
        PDF::Hitable { origin, hitable }
    }

    pub fn mixture(p: &'a PDF, q: &'a PDF) -> Self {
        PDF::Mixture { p, q }
    }

    pub fn value(&self, direction: Vector3<f32>) -> f32 {
        match self {
            PDF::Cosine { uvw } => {
                let cosine = direction.normalize().dot(&uvw.w());
                if cosine > 0.0 {
                    cosine / f32::consts::PI
                } else {
                    1.0
                }
            },
            PDF::Hitable { origin, hitable } => {
                hitable.pdf_value(*origin, direction)
            },
            PDF::Mixture { p, q } => {
                0.5 * p.value(direction) + 0.5 * q.value(direction)
            }
        }
    }

    pub fn generate(&self) -> Vector3<f32> {
        match self {
            PDF::Cosine { uvw } => {
                uvw.local(&random_cosine_direction())
            },
            PDF::Hitable { origin, hitable } => {
                hitable.random(*origin)
            },
            PDF::Mixture { p, q } => {
                let mut rng = rand::thread_rng();
                if rng.gen::<bool>() { p.generate() } else { q.generate() }
            }
        }
    }
}
