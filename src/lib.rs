use rand::Rng;

static mut RAY_COUNT: u128 = 0;
static mut RAY_COUNT_PRIMARY: u128 = 0;
static mut INTERSECT_TESTS: u128 = 0;
static mut INTERSECT_PASSES: u128 = 0;
static mut INTERSECT_TESTS_SP: u128 = 0;
static mut INTERSECT_PASSES_SP: u128 = 0;
static mut INTERSECT_TESTS_AABB: u128 = 0;
static mut INTERSECT_PASSES_AABB: u128 = 0;

mod utils;
mod behaviors;
mod objects;
mod materials;
mod scenes;


use crate::utils::{ Color, Vec3, Ray, pretty_print_int };
use crate::behaviors::Scatter;
use crate::objects::BvhNode;
use crate::materials::Material;


pub fn raytrace() {

    let aspect_ratio = 16.0 / 9.0;
    let width = 800;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let ray_depth = 50;


    let (camera, materials, world) = scenes::spheres(aspect_ratio, 0.3);
    // let (camera, materials, world) = scenes::meshtest(aspect_ratio, 0.0);
    // println!("{:#?}", &world);

    let root = BvhNode::construct(world.objects, 0);
    // println!("{:?}", root);


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
                unsafe { RAY_COUNT_PRIMARY += 1; }
                current_color += ray_color(&root, &materials, ray, ray_depth);
            }

            i = ((height - h - 1) * width) + w;
            buffer[i] = Color::to_u8(current_color, samples_per_pixel);
        }
    }
    println!();

    utils::image_export("image.ppm", &buffer, width, height);
    println!("\n Image exported!");

    unsafe {
        println!("Rays processed:                    {}", pretty_print_int(RAY_COUNT));
        println!("Rays processed(primary):           {}", pretty_print_int(RAY_COUNT_PRIMARY));
        println!("Intersection tests done(Mesh):     {}", pretty_print_int(INTERSECT_TESTS));
        println!("Intersection tests passed(Mesh):   {}", pretty_print_int(INTERSECT_PASSES));
        println!("Intersection tests done(Sphere):   {}", pretty_print_int(INTERSECT_TESTS_SP));
        println!("Intersection tests passed(Sphere): {}", pretty_print_int(INTERSECT_PASSES_SP));
        println!("Intersection tests done(aabb):     {}", pretty_print_int(INTERSECT_TESTS_AABB));
        println!("Intersection tests passed(aabb):   {}", pretty_print_int(INTERSECT_PASSES_AABB));
    }
}

static T_MIN: f64 = 0.0001;
static T_MAX: f64 = f64::INFINITY;


fn ray_color(root: &BvhNode, materials: &Vec<Material>, ray: Ray, depth: usize) -> Vec3 {

    if depth <= 0 { return Vec3::zero() }

    // if let Some(_) = root.bbox().intersect(&ray, T_MIN, T_MAX) {
        if let Some(result) = root.intersect(&ray, T_MIN, T_MAX) {
            let material = &materials[result.material];
            let emitted = material.emit();

            match material.scatter(&ray, result) {
                Some(r) => {
                    let color = ray_color(&root, &materials, r.ray, depth - 1);
                    return emitted + r.attenuation * color
                },
                None => {
                    return emitted
                }
            }
        }
    // }

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
