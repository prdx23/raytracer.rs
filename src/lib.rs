use rayon::prelude::*;

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


use crate::utils::{ Color, Vec3, Ray, Matrix4, pretty_print_int };
use crate::behaviors::{ Intersect, Scatter };
use crate::objects::{ Object, BvhNode };
use crate::materials::Material;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 800;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const RAY_DEPTH: usize = 150;
const MULTICORE: bool = false;
const BIAS: f64 = 0.005;



pub fn raytrace() {

    // let (camera, materials, world) = scenes::spheres(ASPECT_RATIO, 0.3);
    // let (camera, materials, world) = scenes::meshtest(ASPECT_RATIO, 0.2);
    let (camera, materials, world) = scenes::teapot_with_lights(ASPECT_RATIO, 0.15);
    // let (camera, materials, world) = scenes::cubes(ASPECT_RATIO, 0.3);
    // println!("{:#?}", &world);

    let mut primitives: Vec<Object> = vec![];

    for object in world.into_iter() {
        if let Some(inner_objs) = object.divide() {
            primitives.extend(inner_objs);
        } else {
            primitives.push(object);
        }
    }

    let mut objects: Vec<Object> = vec![];
    let mut nodes: Vec<BvhNode> = vec![];
    let root = BvhNode::construct(primitives, &mut objects, &mut nodes);
    // println!("{:?}", root);


    let mut buffer: Vec<Color> = vec![Color::black() ; WIDTH * HEIGHT];

    println!("Starting Render...");
    if !MULTICORE {
        buffer
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| {
                let mut rng = rand::thread_rng();
                let w = (i % WIDTH) as f64;
                let h = (HEIGHT - (i / WIDTH) - 1) as f64;

                print!(
                    "\r Rendering line {}/{} ...", 
                    HEIGHT - h as usize - 1, HEIGHT - 1
                );

                let color: Vec3 = (&mut rng)
                    .sample_iter(rand::distributions::Standard)
                    .take(SAMPLES_PER_PIXEL)
                    .collect::<Vec<(f64, f64)>>()
                    .into_iter()
                    .fold(Vec3::zero(), |c, (a, b)| {
                        let u = (w + a) / WIDTH as f64;
                        let v = (h + b) / HEIGHT as f64;

                        unsafe { RAY_COUNT_PRIMARY += 1; }

                        let ray = camera.get_ray(u, v);
                        let col = ray_color(
                            root, &objects, &materials, &nodes,
                            ray, RAY_DEPTH
                        );

                        c + col
                    });

                *x = Color::to_u8(color, SAMPLES_PER_PIXEL);
            });
    } else {

        buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, x)| {
                let mut rng = rand::thread_rng();
                let w = (i % WIDTH) as f64;
                let h = (HEIGHT - (i / WIDTH) - 1) as f64;

                // print!(
                //     "\r Rendering line {}/{} ...", 
                //     HEIGHT - h as usize - 1, HEIGHT - 1
                // );

                let color: Vec3 = (&mut rng)
                    .sample_iter(rand::distributions::Standard)
                    .take(SAMPLES_PER_PIXEL)
                    .collect::<Vec<(f64, f64)>>()
                    .into_par_iter()
                    .fold(|| Vec3::zero(), |c, (a, b)| {

                        let u = (w + a) / WIDTH as f64;
                        let v = (h + b) / HEIGHT as f64;

                        unsafe { RAY_COUNT_PRIMARY += 1; }

                        let ray = camera.get_ray(u, v);
                        let col = ray_color(
                            root, &objects, &materials, &nodes,
                            ray, RAY_DEPTH
                        );

                        c + col
                    })
                    .reduce(|| Vec3::zero(), |acc, cur| acc + cur);

                *x = Color::to_u8(color, SAMPLES_PER_PIXEL);
            });
    }
    println!();

    println!("Exporting image");
    utils::image_export("image.ppm", &buffer, WIDTH, HEIGHT);
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


fn ray_color(
    root: usize, objects: &[Object], materials: &[Material],
    nodes: &[BvhNode], ray: Ray, depth: usize
) -> Vec3 {

    if depth <= 0 { return Vec3::zero() }

    debug_assert!(root < nodes.len());
    let node = unsafe { nodes.get_unchecked(root) };

    if let Some(result) = node.intersect(&ray, T_MIN, T_MAX, objects, nodes) {
        let material = &materials[result.material];
        let emitted = material.emit();

        match material.scatter(&ray, result) {
            Some(r) => {
                let color = ray_color(
                    root, objects, materials, nodes, r.ray, depth - 1
                );
                return emitted + r.attenuation * color
            },
            None => {
                return emitted
            }
        }
    }

    // Vec3::new(1.0, 1.0, 1.0)
    Vec3::new(0.001, 0.001, 0.001)
    // Vec3::zero()
    // let unit_direction = ray.direction().unit();
    // let t = 0.5 * (unit_direction.y + 1.0);

    // Vec3::new(
    //     utils::lerp(1.0, 0.5, t),
    //     utils::lerp(1.0, 0.7, t),
    //     utils::lerp(1.0, 1.0, t),
    // // ) * 0.001
    // )
}
