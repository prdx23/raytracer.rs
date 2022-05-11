use std::fs;

use crate::Vec3;
use crate::objects::Mesh;

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn rect_hor_rev(v0: Vec3, v1: Vec3, mat: usize) -> Mesh {
    Mesh::new(
        vec![
            v0,
            Vec3::new(v1.x, v1.y, v0.z),
            v1,
            Vec3::new(v0.x, v0.y, v1.z),
        ],
        vec![(0, 0), (2, 0), (1, 0), (0, 0), (3, 0), (2, 0)],
        vec![],
        mat,
    )
}

#[allow(dead_code)]
pub fn cuboid(p: Vec3, l: f64, h: f64, b: f64, mat: usize) -> Mesh {
    Mesh::new(
        vec![
            Vec3::new(p.x + 0.0, p.y + 0.0, p.z - 0.0), // 0
            Vec3::new(p.x + l  , p.y + 0.0, p.z - 0.0), // 1
            Vec3::new(p.x + l  , p.y + h  , p.z - 0.0), // 2
            Vec3::new(p.x + 0.0, p.y + h  , p.z - 0.0), // 3
            Vec3::new(p.x + 0.0, p.y + 0.0, p.z - b  ), // 4
            Vec3::new(p.x + l  , p.y + 0.0, p.z - b  ), // 5
            Vec3::new(p.x + l  , p.y + h  , p.z - b  ), // 6
            Vec3::new(p.x + 0.0, p.y + h  , p.z - b  ), // 7
        ],
        vec![
            (0, 0), (1, 0), (2, 0), (0, 0), (2, 0), (3, 0), // front
            (5, 0), (4, 0), (7, 0), (5, 0), (7, 0), (6, 0), // back
            (1, 0), (5, 0), (6, 0), (1, 0), (6, 0), (2, 0), // right
            (4, 0), (0, 0), (3, 0), (4, 0), (3, 0), (7, 0), // left
            (3, 0), (2, 0), (6, 0), (3, 0), (6, 0), (7, 0), // top
            (4, 0), (5, 0), (1, 0), (4, 0), (1, 0), (0, 0), // bottom
        ],
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
        let opts: Vec<&str> = line.split(" ").filter(|x| *x != "").collect();
        if opts.len() == 0 { continue; }

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
                    if opts[i] == "" { continue; }

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
