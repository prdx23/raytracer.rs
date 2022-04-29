
use crate::utils::{ Vec3, Color, Camera };
use crate::objects::{ World, Sphere, Mesh, helpers };
use crate::materials::{ Material, Lambertian, Metal, Dielectric, DiffuseLight };


pub fn meshtest(aspect_ratio: f64, dof: f64) -> (Camera, Vec<Material>, World) {

    // camera
    let look_from = Vec3::new(0.0, 3.5, 13.0);
    let look_at = Vec3::new(0.5, 0.5, -0.5);
    let camera = Camera::new(
        look_from, look_at,
        Vec3::new(0.0, 1.0, 0.0),
        35.0, aspect_ratio,
        dof, (look_from - look_at).len(),
    );

    // materials
    let materials: Vec<Material> = vec![
        Lambertian::new(Color::rgb(218, 76, 76)),
        Lambertian::grey(),
        // Metal::new(Color::rgb(204, 204, 204), 0.0),
        // Lambertian::new(Color::rgb(76, 76, 218)),
        // Metal::new(Color::rgb(15, 151, 204), 0.3),
        // Dielectric::new(1.5),
        // DiffuseLight::white(10.0),
        // DiffuseLight::new(Color::rgb(255, 0, 0), 20.0),
        // DiffuseLight::new(Color::rgb(15, 151, 204), 20.0),
    ];

    // helpers::from_obj(String::from("teapot.obj"));
    // world
    let mut world = World::new();
    world.add(
        helpers::from_obj(String::from("teapot.obj"), 0)
    );

    // world.add(Triangle {
    //     v0: Vec3::new(-1.0, -0.5, 0.0),
    //     v1: Vec3::new(1.0, -0.5, 0.0),
    //     v2: Vec3::new(0.0, 0.5, 0.0),
    //     material: 0,
    //     doublesided: true,
    // });

    // world.add(TriangleMesh::new(
    //     vec![
    //         Vec3::new(0.0, 0.0, 0.0),
    //         Vec3::new(1.0, 0.0, 0.0),
    //         Vec3::new(1.0, 1.0, 0.0),
    //         Vec3::new(0.0, 1.0, 0.0),
    //     ],
    //     vec![0, 1, 3, 1, 2, 3],
    //     0,
    // ));

    // world.add(helpers::rect(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(1.0, 1.0, 0.0),
    //     0,
    // ));

    // world.add(Sphere {
    //     center: Vec3::new(0.0, 0.0, -1.0),
    //     radius: 0.5,
    //     material: 0,
    // });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: 1,
    });
    // world.add(Sphere {
    //     center: Vec3::new(-1.3, 0.0, -1.0),
    //     radius: 0.5,
    //     material: 2,
    // });
    // world.add(Sphere {
    //     center: Vec3::new(1.2, -0.1, -1.0),
    //     radius: 0.4,
    //     material: 3,
    // });
    // world.add(Sphere {
    //     center: Vec3::new(-0.6, -0.3, -0.5),
    //     radius: 0.2,
    //     material: 4,
    // });
    // world.add(Sphere {
    //     center: Vec3::new(1.2, 0.6, -1.0),
    //     radius: 0.3,
    //     material: 5,
    // });
    // world.add(Sphere {
    //     center: Vec3::new(0.0, 1.2, -1.0),
    //     radius: 0.3,
    //     material: 6,
    // });
    // world.add(Sphere {
    //     center: Vec3::new(-0.0, -0.4, -0.4),
    //     radius: 0.04,
    //     material: 7,
    // });
    // world.add(Sphere {
    //     center: Vec3::new(1.6, -0.4, -1.0),
    //     radius: 0.03,
    //     material: 8,
    // });

    (camera, materials, world)
}
