mod ray;
mod hitable;
mod texture;
mod material;
mod rect;
mod cube;
mod translate;
mod rotate;
mod camera;
mod aabb;

use std::f32;
use nalgebra::Vector3;
use rand::Rng;
use rayon::prelude::*;
use crate::ray::Ray;
use crate::texture::ConstantTexture;
use crate::material::{Lambertian, DiffuseLight};
use crate::hitable::{Hitable, HitableList, FlipNormals};
use crate::rect::{AARect, Plane};
use crate::cube::Cube;
use crate::translate::Translate;
use crate::rotate::{Rotate, Axis};
use crate::camera::Camera;

fn cornell_box(aspect: f32) -> (Box<Hitable>, Camera) {
    let red = Lambertian::new(ConstantTexture::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(ConstantTexture::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(ConstantTexture::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(ConstantTexture::new(15.0, 15.0, 15.0));
    let mut world = HitableList::default();
    world.push(FlipNormals::new(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(AARect::new(Plane::ZX, 227.0, 332.0, 213.0, 343.0, 554.0, light));
    world.push(FlipNormals::new(AARect::new(Plane::ZX, 0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.push(AARect::new(Plane::ZX, 0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    world.push(FlipNormals::new(AARect::new(Plane::XY, 0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.push(
        Translate::new(
            Rotate::new(Axis::Y,
                        Cube::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(165.0, 165.0, 165.0), white.clone()),
                        -18.0),
            Vector3::new(130.0, 0.0, 65.0)));
    world.push(
        Translate::new(
            Rotate::new(Axis::Y,
                        Cube::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(165.0, 330.0, 165.0), white),
                        15.0),
            Vector3::new(265.0, 0.0, 295.0)));

    let look_from = Vector3::new(278.0, 278.0, -800.0);
    let look_at = Vector3::new(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;
    let vertical_fov = 40.0;
    let cam = Camera::new(
        look_from, look_at, Vector3::new(0.0, 1.0, 0.0),
        vertical_fov, aspect, aperture, focus_dist, 0.0, 1.0);

    (Box::new(world), cam)
}

fn color(ray: &Ray, world: &Box<Hitable>, depth: i32) -> Vector3<f32> {
    if let Some(hit) = world.hit(ray, 0.001, f32::MAX) {
        let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
        if depth < 50 {
            if let Some((scattered, attenuation, pdf)) = hit.material.scatter(&ray, &hit) {
                let scattering_pdf = hit.material.scattering_pdf(&ray, &hit, &scattered);
                return emitted + attenuation.zip_map(
                    &(scattering_pdf * color(&scattered, &world, depth+1)), |l, r| l * r) / pdf;
            }
        }
        emitted
    } else {
        Vector3::zeros()
    }
}

fn main() {
    let nx = 500;
    let ny = 500;
    let ns = 1000;
    println!("P3\n{} {}\n255", nx, ny);
    let (world, cam) = cornell_box(nx as f32 / ny as f32);
    let image =
        (0..ny).into_par_iter().rev()
            .flat_map(|y|
                (0..nx).flat_map(|x| {
                    let col: Vector3<f32> = (0..ns).map(|_| {
                        let mut rng = rand::thread_rng();
                        let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
                        let v = (y as f32 + rng.gen::<f32>()) / ny as f32;
                        let ray = cam.get_ray(u, v);
                        color(&ray, &world, 0)
                    }).sum();
                    col.iter().map(|c|
                        (255.99 * (c / ns as f32).sqrt().max(0.0).min(1.0)) as u8
                    ).collect::<Vec<u8>>()
                }).collect::<Vec<u8>>()
            ).collect::<Vec<u8>>();
    for col in image.chunks(3) {
       println!("{} {} {}", col[0], col[1], col[2]);
    }
}
