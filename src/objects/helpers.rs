use std::fs;

use crate::Vec3;
use crate::objects::Mesh;


pub fn rect_ver(v0: Vec3, v1: Vec3, mat: usize) -> Mesh {
    Mesh::new(
        vec![
            v0,
            Vec3::new(v1.x, v0.y, v1.z),
            v1,
            Vec3::new(v0.x, v1.y, v0.z),
        ],
        vec![(0, 0), (1, 0), (3, 0), (1, 0), (2, 0), (3, 0)],
        vec![],
        mat,
    )
}

pub fn rect_hor(v0: Vec3, v1: Vec3, mat: usize) -> Mesh {
    Mesh::new(
        vec![
            v0,
            Vec3::new(v1.x, v1.y, v0.z),
            v1,
            Vec3::new(v0.x, v0.y, v1.z),
        ],
        vec![(0, 0), (1, 0), (2, 0), (0, 0), (2, 0), (3, 0)],
        vec![],
        mat,
    )
}

pub fn from_obj(filename: String, mat: usize) -> Mesh {

    let txt = fs::read_to_string(filename).expect("File not found!");

    let mut vertices: Vec<Vec3> = vec![];
    let mut normals: Vec<Vec3> = vec![];
    let mut indexes: Vec<(usize, usize)> = vec![];
    let mut face_indexes: Vec<usize> = vec![];

    let mut not_trig_mesh = false;

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
            "vn" => {
                normals.push(Vec3::new(
                    opts[1].parse().unwrap(),
                    opts[2].parse().unwrap(),
                    opts[3].parse().unwrap(),
                ))
            },
            "f" => {
                if opts.len() > 4 { not_trig_mesh = true };

                face_indexes.push(opts.len() - 1);

                for i in 1..opts.len() {
                    let values: Vec<&str> = opts[i].split("/").collect();
                    let index: usize = values[0].parse().unwrap();

                    let n_index: usize = if values.len() == 3 {
                        values[2].parse().unwrap()
                    } else { 0 };

                    indexes.push((index - 1, n_index - 1));
                }
            },
            _ => {}
        };
    }

    if not_trig_mesh {
        let mut new_indexes = vec![];
        let mut k = 0;
        for i in 0..face_indexes.len() {
            for j in 0..(face_indexes[i] - 2) {
                new_indexes.push(indexes[k]);
                new_indexes.push(indexes[k + j + 1]);
                new_indexes.push(indexes[k + j + 2]);
            }
            k += face_indexes[i];
        }

        indexes = new_indexes;
    }

    Mesh::new(vertices, indexes, normals, mat)
}
