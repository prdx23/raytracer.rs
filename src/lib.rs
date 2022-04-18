use std::rc::Rc;
use rand::Rng;

mod utils;
mod behaviors;
mod objects;
mod materials;


use crate::utils::{ Color, Vec3, Ray, Camera, };
use crate::behaviors::{ Scatter, };
use crate::objects::{ World, Sphere, };
use crate::materials::{ Lambertian, };


pub fn raytrace() {

    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 30;
    let ray_depth = 50;

    // camera
    let camera = Camera::new();

    // materials
    // let diffuse_material: Rc<dyn Scatter> = Rc::new(Lambertian::grey());
    let diffuse_material = Lambertian::grey().rc();

    // world
    let mut world = World::new();
    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::clone(&diffuse_material),
    });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::clone(&diffuse_material),
    });
    println!("{:#?}", &world);

    // pixel buffer
    let mut buffer: Vec<Color> = vec![Color::new() ; width * height];


    let mut i;
    let mut u: f64;
    let mut v: f64;

    let mut ray: Ray;
    let mut current_color: Vec3;
    let mut rng = rand::thread_rng();

    for h in (0..height).rev() {
        for w in 0..width {
            print!("\r Rendering line {}/{} ...", height - h - 1, height - 1);

            current_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                u = ((w as f64) + rng.gen::<f64>()) / width as f64;
                v = ((h as f64) + rng.gen::<f64>()) / height as f64;

                ray = camera.get_ray(u, v);
                current_color += ray_color(&world, ray, ray_depth);
            }

            i = ((height - h - 1) * width) + w;
            buffer[i] = Color::normalize(current_color, samples_per_pixel);
        }
    }
    println!();

    utils::image_export("image.ppm", &buffer, width, height);
    println!("\n Image exported!");
}


fn ray_color(world: &World, ray: Ray, depth: usize) -> Vec3 {

    if depth <= 0 { return Vec3::zero() }

    if let Some(r) = world.find_intersection(&ray, 0.00001, f64::INFINITY) {
        return r.attenuation * ray_color(&world, r.ray, depth - 1)
        // return Vec3::new(
        //     (detail.normal().x + 1.0) * 0.5,
        //     (detail.normal().y + 1.0) * 0.5,
        //     (detail.normal().z + 1.0) * 0.5,
        // )
    }

    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    Vec3::new(
        utils::lerp(1.0, 0.5, t),
        utils::lerp(1.0, 0.7, t),
        utils::lerp(1.0, 1.0, t),
    )
}
