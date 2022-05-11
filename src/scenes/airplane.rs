
use crate::utils::{ Vec3, Color, Camera };
use crate::objects::{ Object, Sphere, helpers };
use crate::materials::{ Material, Lambertian, Metal, Dielectric, DiffuseLight };


#[allow(dead_code)]
pub fn airplane(aspect_ratio: f64, dof: f64)
    -> (Camera, Vec<Material>, Vec<Object>)
{

    // camera
    let look_from = Vec3::new(-180.0, 65.0, 130.0);
    let look_at = Vec3::new(0.0, 20.0, -5.0);
    let camera = Camera::new(
        look_from, look_at,
        Vec3::new(0.0, 1.0, 0.0),
        25.0, aspect_ratio,
        dof, (look_from - look_at).len(),
    );

    // materials
    let materials: Vec<Material> = vec![
        // Lambertian::grey(),
        Metal::new(Color::rgb(104, 104, 104), 0.9),
        DiffuseLight::new(Color::rgb(216, 181, 75), 10.0),

        DiffuseLight::new(Color::rgb(1, 255, 1), 50.0),
        DiffuseLight::new(Color::rgb(255, 10, 10), 50.0),

    ];

    let mut airplane = helpers::from_obj(String::from("objs/airplane.obj"), 0);
    airplane.scale(0.1, 0.1, 0.1);
    airplane.rotate_x(-90.0);
    airplane.rotate_y(180.0);

    let world: Vec<Object> = vec![
        airplane.into(),

        Sphere {
            center: Vec3::new(130.0, 85.0, 200.0),
            radius: 120.0,
            material: 1,
        }.into(),

        Sphere {
            center: Vec3::new(-78.5, 22.0, -22.0),
            radius: 0.5,
            material: 2,
        }.into(),

        Sphere {
            center: Vec3::new(78.5, 22.0, -22.0),
            radius: 0.5,
            material: 3,
        }.into(),

    ];

    (camera, materials, world)
}
