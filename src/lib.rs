use rand::Rng;

mod utils;
mod behaviors;
mod objects;
mod materials;


use crate::utils::{ Color, Vec3, Ray, Camera, };
use crate::behaviors::{ Intersect, Scatter, IntersectResult };
use crate::objects::{ World, Sphere, Triangle };
use crate::materials::{ Material, Lambertian, Metal, Dielectric, DiffuseLight };


pub fn raytrace() {

    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 10;
    let ray_depth = 50;

    // camera
    let look_from = Vec3::new(7.0, 1.3, 3.2);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let camera = Camera::new(
        look_from, look_at,
        Vec3::new(0.0, 1.0, 0.0),
        15.0, aspect_ratio,
        0.0, (look_from - look_at).len(),
    );

    let materials: Vec<Material> = vec![
        Lambertian::new(Color::rgb(218, 76, 76)),
        Lambertian::grey(),
        Metal::new(Color::rgb(204, 204, 204), 0.0),
        Lambertian::new(Color::rgb(76, 76, 218)),
        Metal::new(Color::rgb(15, 151, 204), 0.3),
        Dielectric::new(1.5),
        // DiffuseLight::white(10.0),
        // DiffuseLight::new(Color::rgb(255, 0, 0), 20.0),
        // DiffuseLight::new(Color::rgb(15, 151, 204), 20.0),
    ];

    // world
    let mut world = World::new();
    world.add(Triangle {
        v0: Vec3::new(-1.0, -0.5, 0.0),
        v1: Vec3::new(1.0, -0.5, 0.0),
        v2: Vec3::new(0.0, 0.5, 0.0),
        material: 0,
        doublesided: true,
    }.into());

    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: 0,
    }.into());
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: 1,
    }.into());
    world.add(Sphere {
        center: Vec3::new(-1.3, 0.0, -1.0),
        radius: 0.5,
        material: 2,
    }.into());
    world.add(Sphere {
        center: Vec3::new(1.2, -0.1, -1.0),
        radius: 0.4,
        material: 3,
    }.into());
    world.add(Sphere {
        center: Vec3::new(-0.6, -0.3, -0.5),
        radius: 0.2,
        material: 4,
    }.into());
    world.add(Sphere {
        center: Vec3::new(1.2, 0.6, -1.0),
        radius: 0.3,
        material: 5,
    }.into());
    // world.add(Sphere {
    //     center: Vec3::new(0.0, 1.2, -1.0),
    //     radius: 0.3,
    //     material: 6,
    // }.into());
    // world.add(Sphere {
    //     center: Vec3::new(-0.0, -0.4, -0.4),
    //     radius: 0.04,
    //     material: 7,
    // }.into());
    // world.add(Sphere {
    //     center: Vec3::new(1.6, -0.4, -1.0),
    //     radius: 0.03,
    //     material: 8,
    // }.into());
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
                current_color += ray_color(&world, &materials, ray, ray_depth);
            }

            i = ((height - h - 1) * width) + w;
            buffer[i] = Color::to_u8(current_color, samples_per_pixel);
        }
    }
    println!();

    utils::image_export("image.ppm", &buffer, width, height);
    println!("\n Image exported!");
}


fn ray_color(world: &World, materials: &Vec<Material>, ray: Ray, depth: usize) -> Vec3 {

    if depth <= 0 { return Vec3::zero() }

    if let Some((i, t)) = world.find_intersection(&ray, 0.0001, f64::INFINITY) {

        let obj = &world.objects[i];

        let point = ray.at(t);
        let normal = obj.get_intersect_normal(&ray, t);

        let intersect_result = IntersectResult::new(point, &ray, normal);

        let mat = obj.material(materials);
        let emitted = mat.emit();

        match mat.scatter(&ray, intersect_result) {
            Some(r) => {
                return emitted + r.attenuation * ray_color(&world, &materials, r.ray, depth - 1)
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
    // ) * 0.001
    )
}
