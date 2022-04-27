use std::fs;

use crate::Vec3;
use crate::objects::Mesh;


pub fn rect(v0: Vec3, v1: Vec3, mat: usize) -> Mesh {
    Mesh::new(
        vec![
            v0,
            Vec3::new(v1.x, v0.y, v1.z),
            v1,
            Vec3::new(v0.x, v1.y, v0.z),
        ],
        vec![0, 1, 3, 1, 2, 3],
        mat,
    )
}


pub fn from_obj(filename: String, mat: usize) -> Mesh {

    let txt = fs::read_to_string(filename).expect("File not found!");

    let mut vertices: Vec<Vec3> = vec![];
    let mut indices: Vec<usize> = vec![];

    for line in txt.lines() {
        let opts: Vec<&str> = line.split(" ").collect();
        match opts[0] {
            "v" => {
                vertices.push(Vec3::new(
                    opts[1].parse().unwrap(),
                    opts[2].parse().unwrap(),
                    opts[3].parse().unwrap(),
                ))
            },
            "f" => {
                for i in 1..opts.len() {
                    let index: usize = opts[i]
                        .split("/")
                        .collect::<Vec<&str>>()[0]
                        .parse()
                        .unwrap();

                    indices.push(index - 1)
                }
            },
            _ => {}
        };
    }

    // println!("{:?}", vertices);
    // println!("{:?}", indices);
    Mesh::new(vertices, indices, mat)
}
