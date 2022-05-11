
use crate::utils::{ Vec3, Color, Camera };
use crate::objects::{ Object, helpers };
use crate::materials::{ Material, Lambertian, Metal, DiffuseLight };


#[allow(dead_code)]
pub fn cornell_box(aspect_ratio: f64, dof: f64)
    -> (Camera, Vec<Material>, Vec<Object>)
{

    // camera
    let look_from = Vec3::new(0.0, 50.0, 230.0);
    let look_at = Vec3::new(0.0, 50.0, 0.0);
    let camera = Camera::new(
        look_from, look_at,
        Vec3::new(0.0, 1.0, 0.0),
        25.0, aspect_ratio,
        dof, (look_from - look_at).len(),
    );

    // materials
    let materials: Vec<Material> = vec![
        DiffuseLight::white(12.0),

        Lambertian::new(Color::rgb(255, 255, 255)),
        Lambertian::new(Color::rgb(211, 16, 16)),
        Lambertian::new(Color::rgb(16, 211, 211)),

        Lambertian::new(Color::rgb(209, 135, 54)),
        Metal::new(Color::rgb(4, 6, 23), 0.0),

        Lambertian::grey(),
    ];

    let mut world: Vec<Object> = vec![];

    let size = 100.0;

    // back wall
    let mut wall = helpers::rect_ver(Vec3::zero(), Vec3::new(size, size, 0.0), 1);
    wall.translate(-size / 2.0, 0.0, -size);
    world.push(wall.into());

    // left wall
    let mut wall = helpers::rect_ver(Vec3::zero(), Vec3::new(size, size, 0.0), 2);
    wall.rotate_y(90.0);
    wall.translate(-size / 2.0, 0.0, 0.0);
    world.push(wall.into());

    // right wall
    let mut wall = helpers::rect_ver(Vec3::zero(), Vec3::new(size, size, 0.0), 3);
    wall.rotate_y(-90.0);
    wall.translate(size / 2.0, 0.0, -size);
    world.push(wall.into());

    // ceiling
    let mut ceiling = helpers::rect_hor(Vec3::zero(), Vec3::new(size, 0.0, -size), 1);
    ceiling.rotate_z(180.0);
    ceiling.translate(size / 2.0, size, 0.0);
    world.push(ceiling.into());

    // floor
    let mut floor = helpers::rect_hor(Vec3::zero(), Vec3::new(size, 0.0, -size), 1);
    floor.translate(-size / 2.0, 0.0, 0.0);
    world.push(floor.into());


    let mut tallbox = helpers::cuboid(
        Vec3::zero(), size * 0.3, size * 0.6, size * 0.3, 5
    );
    tallbox.rotate_y(20.0);
    tallbox.translate(-size / 4.0, 0.0, -size / 2.0);
    world.push(tallbox.into());

    let mut smallbox = helpers::cuboid(
        Vec3::zero(), size * 0.3, size * 0.3, size * 0.3, 4
    );
    smallbox.rotate_y(-15.0);
    smallbox.translate(0.0, 0.0, -size / 4.0);
    world.push(smallbox.into());

    // light
    let mut light = helpers::rect_hor(
        Vec3::zero(), Vec3::new(size * 0.3, 0.0, -size * 0.3), 0
    );
    light.rotate_z(180.0);
    light.translate(0.0, size - 0.008, -size / 2.0);
    light.translate(size * 0.15, 0.0, size * 0.15);
    world.push(light.into());


    (camera, materials, world)
}
