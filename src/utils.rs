use std::fs;

pub mod color;
pub mod vector;
pub mod ray;
pub mod camera;

pub use color::Color;
pub use vector::Vec3;
pub use ray::Ray;
pub use camera::Camera;


pub fn image_export(
    name: &str, buffer: &Vec<Color>, width: usize, height: usize
) {

    let mut body = String::new();
    let mut i:usize;

    for h in 0..height {
        print!("\r Writing line {}/{} ...", h, height);

        for w in 0..width {
            i = (h * width) + w;
            let color = &buffer[i];
            body.push_str(&format!("{} {} {} ", color.r, color.g, color.b));

        }
        body.push_str("\n");
    }

    // dbg!(&body);
    fs::write(name, format!("P3\n{} {}\n256\n{}", width, height, body))
        .expect("Unable to write image file");
}

pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    (1.0 - t) * start + t * end
    // start + t * (end - start)
}


pub fn pretty_print_int(i: u128) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}
