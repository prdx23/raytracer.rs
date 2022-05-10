
use crate::utils::{ Vec3, Color, Camera };
use crate::objects::{ Object, Sphere, helpers };
use crate::materials::{ Material, Lambertian, Metal, Dielectric, DiffuseLight };


#[allow(dead_code)]
pub fn teapot_with_lights(aspect_ratio: f64, dof: f64)
    -> (Camera, Vec<Material>, Vec<Object>)
{

    // camera
    let look_from = Vec3::new(-6.0, 4.0, -15.0);
    let look_at = Vec3::new(0.0, 1.4, 0.0);
    let camera = Camera::new(
        look_from, look_at,
        Vec3::new(0.0, 1.0, 0.0),
        25.0, aspect_ratio,
        dof, (look_from - look_at).len(),
    );

    // materials
    let materials: Vec<Material> = vec![
        Lambertian::grey(),

        Metal::new(Color::rgb(204, 204, 204), 0.2),

        Lambertian::new(Color::rgb(118, 168, 188)),

        Dielectric::new(1.5),

        DiffuseLight::white(4.0),
        DiffuseLight::new(Color::rgb(15, 151, 204), 4.0),
        DiffuseLight::new(Color::rgb(218, 76, 76), 4.0),
        DiffuseLight::new(Color::rgb(216, 181, 75), 4.0),

        Metal::new(Color::rgb(15, 151, 204), 0.7),
        Metal::new(Color::rgb(218, 76, 76), 0.7),
        Metal::new(Color::rgb(216, 181, 75), 0.7),

    ];

    let teapot = helpers::from_obj(String::from("teapot.obj"), 1);
    // teapot.translate(0.5, 0.0, 0.0);
    // teapot.scale_y(2.0);
    // teapot.rotate_z(45.0);
    // teapot.rotate_y(15.0);

    let world: Vec<Object> = vec![

        teapot.into(),

        // table surface
        helpers::rect_hor(
            Vec3::new(-20.0, 0.0, 20.0),
            Vec3::new(40.0, 0.0, -20.0),
            2,
        ).into(),

        // Sphere {
        //     center: Vec3::new(-2.0, 10.0, 3.5),
        //     radius: 5.0,
        //     material: 4,
        // }.into(),

        // metal sphere
        Sphere {
            center: Vec3::new(-3.0, 1.0, 3.5),
            radius: 1.0,
            material: 1,
        }.into(),

        // glass sphere
        Sphere {
            center: Vec3::new(2.0, 1.0, -3.5),
            radius: 1.0,
            material: 3,
        }.into(),

        // lights
        Sphere {
            center: Vec3::new(5.0, 9.0, 5.0),
            radius: 3.0,
            material: 7,
        }.into(),

        Sphere {
            center: Vec3::new(-5.0, 9.0, 5.0),
            radius: 3.0,
            material: 6,
        }.into(),

        Sphere {
            center: Vec3::new(-5.0, 9.0, -5.0),
            radius: 3.0,
            material: 4,
        }.into(),

        Sphere {
            center: Vec3::new(5.0, 9.0, -5.0),
            radius: 3.0,
            material: 5,
        }.into(),

        // small colored spheres
        Sphere {
            center: Vec3::new(-2.5, 0.4, -3.0),
            radius: 0.4,
            material: 8,
        }.into(),

        Sphere {
            center: Vec3::new(-3.0, 0.4, -2.0),
            radius: 0.4,
            material: 9,
        }.into(),

        Sphere {
            center: Vec3::new(-2.0, 0.4, -2.0),
            radius: 0.4,
            material: 10,
        }.into(),

    ];

    (camera, materials, world)
}
