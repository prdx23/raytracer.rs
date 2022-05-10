
use crate::utils::{ Vec3, Color, Camera };
use crate::objects::{ Object, Sphere, helpers };
use crate::materials::{ Material, Lambertian, Metal, Dielectric, DiffuseLight };

use rand::{Rng, SeedableRng};

pub fn cubes(aspect_ratio: f64, dof: f64)
    -> (Camera, Vec<Material>, Vec<Object>)
{

    // camera
    let look_from = Vec3::new(12.0, 10.0, 12.0);
    let look_at = Vec3::new(0.0, 0.4, 0.0);
    let camera = Camera::new(
        look_from, look_at,
        Vec3::new(0.0, 1.0, 0.0),
        // 30.0, aspect_ratio,
        80.0, aspect_ratio,
        dof, (look_from - look_at).len(),
    );

    // materials
    let materials: Vec<Material> = vec![
        // cube color
        Lambertian::new(Color::rgb(170, 168, 180)),

        // Lambertian::new(Color::rgb(118, 168, 188)),
        DiffuseLight::white(1.0),

        // ground light
        DiffuseLight::new(Color::rgb(15, 151, 204), 8.0),
        // DiffuseLight::new(Color::rgb(218, 6, 6), 8.0),


        // circle lights
        DiffuseLight::new(Color::rgb(218, 6, 6), 50.0),
        // DiffuseLight::new(Color::rgb(15, 151, 204), 50.0),
        // DiffuseLight::new(Color::rgb(247, 141, 1), 50.0),

        // sun light
        DiffuseLight::new(Color::rgb(247, 141, 1), 6.0),
        // DiffuseLight::new(Color::rgb(15, 151, 204), 6.0),
        // DiffuseLight::new(Color::rgb(218, 6, 6), 6.0),

        // Dielectric::new(1.5),

        Lambertian::new(Color::rgb(15, 15, 18)),
        // Lambertian::new(Color::rgb(215, 215, 215)),


    ];

    let mut world: Vec<Object> = vec![];

    let side = 2.0;
    let margin = 0.4;
    let mut rng = rand::rngs::SmallRng::seed_from_u64(24);

    let startx = -25.0;
    let nx = 15;
    let startz = 12.0;
    let nz = 16;

    for i in 0..nx {
        for j in 0..nz {
            let x = startx + i as f64 * (side + margin);
            let z = startz + -j as f64 * (side + margin);

            let h = side + rng.gen_range(-1.5..1.5);

            world.push(helpers::cuboid(
                Vec3::new(x, 0.0, z),
                side, h, side,
                0,
            ).into());

            if rng.gen::<f64>() < 0.25 {

                world.push(helpers::cuboid(
                    Vec3::new(
                        x + (side / 2.0) - 0.03,
                        h,
                        z - (side / 2.0) + 0.03,
                    ),
                    0.06, 0.4, 0.06,
                    5,
                ).into());

                world.push( Sphere {
                    center: Vec3::new(
                        x + (side / 2.0),
                        h + 0.4,
                        z - (side / 2.0),
                    ),
                    radius: 0.05,
                    material: 3,
                }.into());
            }
        }
    }

    world.push(helpers::rect_hor(
        Vec3::new(startx, -0.0, startz),
        Vec3::new(
            startx + (nx as f64 * (side + margin)),
            -0.0,
            startz - (nz as f64 * (side + margin))
        ),
        2,
    ).into());


    world.push( Sphere {
        center: Vec3::new(startx + 0.0, 12.0, startz + 0.0),
        // radius: 8.0,
        radius: 15.0,
        material: 4,
    }.into());

    // world.push(helpers::rect_hor_rev(
    //     Vec3::new(startx, 10.0, startz),
    //     Vec3::new(
    //         startx + (nx as f64 * (side + margin)),
    //         10.0,
    //         startz - (nz as f64 * (side + margin))
    //     ),
    //     3,
    // ).into());

    (camera, materials, world)
}
