
use crate::utils::{ Vec3, Color, Camera };
use crate::objects::{ Object, Sphere, helpers };
use crate::materials::{ Material, Lambertian, Metal, Dielectric, DiffuseLight };


#[allow(dead_code)]
pub fn temple(aspect_ratio: f64, dof: f64)
    -> (Camera, Vec<Material>, Vec<Object>)
{

    // camera
    let look_from = Vec3::new(-100.0, 25.0, 120.0);
    let look_at = Vec3::new(0.0, 17.5, 0.0);
    let camera = Camera::new(
        look_from, look_at,
        Vec3::new(0.0, 1.0, 0.0),
        15.0, aspect_ratio,
        // 65.0, aspect_ratio,
        dof, (look_from - look_at).len() - 5.0,
    );

    // materials
    let materials: Vec<Material> = vec![
        // Lambertian::grey(),
        Lambertian::new(Color::rgb(118, 168, 188)),
        // Lambertian::new(Color::rgb(255, 255, 255)),
        Lambertian::grey(),


        DiffuseLight::new(Color::rgb(255, 255, 255), 9.0),
        DiffuseLight::new(Color::rgb(216, 75, 80), 9.0),
        DiffuseLight::new(Color::rgb(22, 180, 190), 7.0),

    ];

    let temple = helpers::from_obj(String::from("objs/temple.obj"), 0);

    let floor = helpers::rect_hor(
        Vec3::new(-40.0, 0.0, 40.0),
        Vec3::new(80.0, 0.0, -80.0),
        1,
    );

    let mut wall1 = helpers::rect_ver(
        Vec3::zero(),
        Vec3::new(120.0, 60.0, 0.0),
        1,
    );
    wall1.translate(-40.0, 0.0, -80.0);

    let mut wall2 = helpers::rect_ver(
        Vec3::zero(),
        Vec3::new(120.0, 60.0, 0.0),
        1,
    );
    wall2.rotate_y(-90.0);
    wall2.translate(80.0, 0.0, -80.0);


    let world: Vec<Object> = vec![

        temple.into(),

        floor.into(),
        wall1.into(),
        wall2.into(),

        // Sphere {
        //     center: Vec3::new(-40.0, 80.0, 40.0),
        //     radius: 15.0,
        //     material: 2,
        // }.into(),

        Sphere {
            center: Vec3::new(10.0, 50.0, 60.0),
            radius: 22.0,
            material: 2,
        }.into(),

        Sphere {
            center: Vec3::new(20.0, 60.0, -20.0),
            radius: 15.0,
            material: 4,
        }.into(),

        Sphere {
            center: Vec3::new(-60.0, 40.0, -00.0),
            radius: 10.0,
            material: 3,
        }.into(),

        // Sphere {
        //     center: Vec3::new(-78.5, 22.0, -22.0),
        //     radius: 0.5,
        //     material: 2,
        // }.into(),

        // Sphere {
        //     center: Vec3::new(78.5, 22.0, -22.0),
        //     radius: 0.5,
        //     material: 3,
        // }.into(),

    ];

    (camera, materials, world)
}
