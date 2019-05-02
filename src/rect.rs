use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord};
use crate::material::Material;
use crate::aabb::AABB;

pub enum Plane {
    YZ,
    ZX,
    XY
}

pub struct AARect<M: Material> {
    plane: Plane,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    material: M
}

fn get_axis(plane: &Plane) -> (usize, usize, usize) {
    match plane {
        Plane::YZ => (0, 1, 2),
        Plane::ZX => (1, 2, 0),
        Plane::XY => (2, 0, 1)
    }
}

impl<M: Material> AARect<M> {
    pub fn new(plane: Plane, a0: f32, a1: f32, b0: f32, b1: f32, k: f32, material: M) -> Self {
        AARect { plane, a0, a1, b0, b1, k, material }
    }
}

impl<M: Material> Hitable for AARect<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (k_axis, a_axis, b_axis) = get_axis(&self.plane);
        let t = (self.k - ray.origin()[k_axis]) / ray.direction()[k_axis];
        if t < t_min || t > t_max {
            None
        } else {
            let a = ray.origin()[a_axis] + t * ray.direction()[a_axis];
            let b = ray.origin()[b_axis] + t * ray.direction()[b_axis];
            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                None
            } else {
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);
                let p = ray.point_at_parameter(t);
                let mut normal = Vector3::zeros();
                normal[k_axis] = 1.0;
                Some(HitRecord { t, u, v, p, normal, material: &self.material })
            }
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = Vector3::new(self.a0, self.b0, self.k - 0.0001);
        let max = Vector3::new(self.a1, self.b1, self.k + 0.0001);
        Some(AABB { min, max })
    }

    fn pdf_value(&self, o: Vector3<f32>, v: Vector3<f32>) -> f32 {
        if let Some(hit) = self.hit(&Ray::new(o, v, 0.0), 0.001, f32::MAX) {
            let area = (self.a1 - self.a0) * (self.b1 - self.b0);
            let distance_squared = hit.t.powi(2) * v.norm_squared();
            let cosine = v.dot(&hit.normal).abs() / v.norm();
            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }

    fn random(&self, o: Vector3<f32>) -> Vector3<f32> {
        let mut rng = rand::thread_rng();
        let (k_axis, a_axis, b_axis) = get_axis(&self.plane);
        let mut random_point = Vector3::zeros();
        random_point[a_axis] = rng.gen_range(&self.a0, &self.a1);
        random_point[b_axis] = rng.gen_range(&self.b0, &self.b1);
        random_point[k_axis] = self.k;
        random_point - o
    }
}