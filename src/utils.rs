pub mod color;
pub mod vector;
pub mod ray;

use std::fs;
// use std::io::Write;
// use std::io::stdout;


use crate::Color;


pub fn image_export(
    name: &str, buffer: &Vec<Color>, width: usize, height: usize
) {

    let mut body = String::new();
    let mut i:usize;

    for h in 0..height {
        print!("\r Writing line {} ...", h);
        // stdout().flush().expect("error");

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
