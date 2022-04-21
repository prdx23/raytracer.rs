use std::rc::Rc;
use rand::Rng;

mod utils;
mod behaviors;
mod objects;
mod materials;


use crate::utils::{ Color, Vec3, Ray, Camera, };
use crate::behaviors::{ Scatter, IntersectResult };
use crate::objects::{ World, Sphere, };
use crate::materials::{ Lambertian, Metal, Dielectric, DiffuseLight };


pub fn raytrace() {

    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 1920;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 3000;
    let ray_depth = 500;

    // camera
    let look_from = Vec3::new(7.0, 1.3, 3.2);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let camera = Camera::new(
        look_from, look_at,
        Vec3::new(0.0, 1.0, 0.0),
        15.0, aspect_ratio,
        0.3, (look_from - look_at).len(),
    );

    // materials
    let diffuse_mat = Lambertian::new(Color::rgb(218, 76, 76)).rc();

    // world
    let mut world = World::new();
    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::clone(&diffuse_mat),
    });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Lambertian::grey().rc(),
    });
    world.add(Sphere {
        center: Vec3::new(-1.3, 0.0, -1.0),
        radius: 0.5,
        material: Metal::new(Color::rgb(204, 204, 204), 0.0).rc(),
    });
    world.add(Sphere {
        center: Vec3::new(1.2, -0.1, -1.0),
        radius: 0.4,
        material: Lambertian::new(Color::rgb(76, 76, 218)).rc(),
    });
    world.add(Sphere {
        center: Vec3::new(-0.6, -0.3, -0.5),
        radius: 0.2,
        material: Metal::new(Color::rgb(15, 151, 204), 0.3).rc(),
    });
    world.add(Sphere {
        center: Vec3::new(1.2, 0.6, -1.0),
        radius: 0.3,
        material: Dielectric::new(1.5).rc(),
    });
    world.add(Sphere {
        center: Vec3::new(0.0, 1.2, -1.0),
        radius: 0.3,
        material: DiffuseLight::white(10.0).rc(),
    });
    world.add(Sphere {
        center: Vec3::new(-0.0, -0.4, -0.4),
        radius: 0.04,
        material: DiffuseLight::new(Color::rgb(255, 0, 0), 20.0).rc(),
    });
    world.add(Sphere {
        center: Vec3::new(1.6, -0.4, -1.0),
        radius: 0.03,
        material: DiffuseLight::new(Color::rgb(15, 151, 204), 20.0).rc(),
    });
    println!("{:#?}", &world);

    // pixel buffer
    let mut buffer: Vec<Color> = vec![Color::black() ; width * height];


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
            buffer[i] = Color::to_u8(current_color, samples_per_pixel);
        }
    }
    println!();

    utils::image_export("image.ppm", &buffer, width, height);
    println!("\n Image exported!");
}


fn ray_color(world: &World, ray: Ray, depth: usize) -> Vec3 {

    if depth <= 0 { return Vec3::zero() }

    if let Some((i, t)) = world.find_intersection(&ray, 0.0001, f64::INFINITY) {

        let obj = &world.objects[i];

        let point = ray.at(t);
        let normal = obj.get_intersect_normal(&ray, t);

        let intersect_result = IntersectResult::new(point, &ray, normal);

        let emitted = obj.material().emit();

        match obj.material().scatter(&ray, intersect_result) {
            Some(r) => {
                return emitted + r.attenuation * ray_color(&world, r.ray, depth - 1)
            },
            None => {
                return emitted
            }
        }
    }

    // Vec3::zero()
    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    Vec3::new(
        utils::lerp(1.0, 0.5, t),
        utils::lerp(1.0, 0.7, t),
        utils::lerp(1.0, 1.0, t),
    ) * 0.001
}
