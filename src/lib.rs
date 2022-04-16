
mod utils;
mod objects;


use crate::utils::color::Color;
use crate::utils::vector::Vec3;
use crate::utils::ray::Ray;
use crate::objects::{Hit, world::World};
use crate::objects::sphere::Sphere;


pub fn raytrace() {

    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as usize;

    // viewport
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // camera
    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let focal_vector = Vec3::new(0.0, 0.0, focal_length);
    let lower_left = origin - (horizontal/2.0) - (vertical/2.0) - focal_vector;

    // world
    let mut world = World::new();
    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });
    // world.add(Cube {
    //     center: Vec3::new(0.0, 0.0, -1.0),
    //     side1: 0.5,
    //     side2: 0.5,
    // });
    // println!("{:?}", world.objects());
    // println!("{:?}", world.objects().iter()[0]);

    // for object in world.objects().iter() {
    //     println!("{:?}", *object.center);
    // }

    let mut buffer: Vec<Color> = vec![Color::new() ; width * height];

    let mut i;
    let mut u: f64;
    let mut v: f64;
    let mut ray: Ray;
    for h in (0..height).rev() {
        for w in 0..width {
            u = w as f64 / width as f64;
            v = h as f64 / height as f64;

            ray = Ray {
                origin: origin,
                direction: lower_left + (horizontal * u) + (vertical * v),
            };

            i = ((height - h - 1) * width) + w;
            buffer[i] = ray_color(&world, ray);
        }
    }

    utils::image_export("image.ppm", &buffer, width, height);
    println!("\n Image exported!");
}


fn ray_color(world: &World, ray: Ray) -> Color {

    if let Some(detail) = world.hit(&ray, 0.0, f64::INFINITY) {
        return Color {
            r: (((detail.normal().x + 1.0) * 0.5) * 255.0) as u8,
            g: (((detail.normal().y + 1.0) * 0.5) * 255.0) as u8,
            b: (((detail.normal().z + 1.0) * 0.5) * 255.0) as u8,
        }
    }

    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    Color {
        r: (utils::lerp(1.0, 0.5, t) * 255.0) as u8,
        g: (utils::lerp(1.0, 0.7, t) * 255.0) as u8,
        b: (utils::lerp(1.0, 1.0, t) * 255.0) as u8,
    }
}
