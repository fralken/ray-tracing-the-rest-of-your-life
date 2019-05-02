use nalgebra::Vector3;

pub struct ONB {
    axis: [Vector3<f32>; 3]
}

impl ONB {
    pub fn build_from_w(n: &Vector3<f32>) -> Self {
        let w = n.normalize();
        let a = if w.x.abs() > 0.9 { Vector3::new(0.0, 1.0, 0.0) } else { Vector3::new(1.0, 0.0, 0.0) };
        let v = w.cross(&a).normalize();
        let u = w.cross(&v);
        ONB { axis: [u, v, w] }
    }

    pub fn u(&self) -> Vector3<f32> { self.axis[0] }
    pub fn v(&self) -> Vector3<f32> { self.axis[1] }
    pub fn w(&self) -> Vector3<f32> { self.axis[2] }

    pub fn local(&self, a: &Vector3<f32>) -> Vector3<f32> {
        a.x * self.u() + a.y * self.v() + a.z * self.w()
    }
}
