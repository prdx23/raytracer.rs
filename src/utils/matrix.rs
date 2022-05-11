use std::ops::{
    Mul,
};


use crate::Vec3;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    pub matrix: [[f64; 4]; 4],
}

impl Matrix4 {

    pub fn new(m: [[f64; 4]; 3]) -> Self {
        Self { matrix: [m[0], m[1], m[2], [0.0, 0.0, 0.0, 1.0]] }
    }

    pub fn translate(tx: f64, ty: f64, tz: f64) -> Self {
        Self::new([
            [1.0, 0.0, 0.0, tx],
            [0.0, 1.0, 0.0, ty],
            [0.0, 0.0, 1.0, tz],
        ])
    }

    pub fn scale(sx: f64, sy: f64, sz: f64) -> Self {
        Self::new([
            [sx, 0.0, 0.0, 0.0],
            [0.0, sy, 0.0, 0.0],
            [0.0, 0.0, sz, 0.0],
        ])
    }

    pub fn rotate_x(theta: f64) -> Self {
        let sin = theta.to_radians().sin();
        let cos = theta.to_radians().cos();
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin, cos, 0.0],
        ])
    }

    pub fn rotate_y(theta: f64) -> Self {
        let sin = theta.to_radians().sin();
        let cos = theta.to_radians().cos();
        Self::new([
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
        ])
    }

    pub fn rotate_z(theta: f64) -> Self {
        let sin = theta.to_radians().sin();
        let cos = theta.to_radians().cos();
        Self::new([
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ])
    }

    pub fn cofactor(
        mat: [[f64; 4]; 4], p: usize, q: usize, n: usize
    ) -> [[f64; 4]; 4] {

        let mut temp = [[0.0; 4]; 4];

        let mut i = 0;
        let mut j = 0;

        for row in 0..n {
            for col in 0..n {
                if row != p && col != q {
                    temp[i][j] = mat[row][col];
                    j += 1;
                    if j == n - 1 {
                        j = 0;
                        i += 1;
                    }
                }
            }
        }

        temp
    }


    pub fn determinant(mat: [[f64; 4]; 4], n: usize) -> f64 {
        if n == 1 { return mat[0][0] }

        let mut det = 0.0;
        let mut sign = 1.0;
        for f in 0..n {
            let cofactor = Matrix4::cofactor(mat, 0, f, n);
            det += sign * mat[0][f] * Matrix4::determinant(cofactor, n - 1);
            sign = sign * -1.0;
        }
        det
    }


    pub fn adjoint(mat: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
        let mut adj = [[0.0; 4]; 4];
        let mut sign;
        for i in 0..4 {
            for j in 0..4 {
                let cofactor = Matrix4::cofactor(mat, i, j, 4);
                sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
                adj[j][i] = sign * Matrix4::determinant(cofactor, 3);
            }
        }
        adj
    }


    pub fn inverse(&self) -> Self {
        let mut inverse = [[0.0; 4]; 4];
        let det = Matrix4::determinant(self.matrix, 4);
        let adj = Matrix4::adjoint(self.matrix);

        debug_assert!(det > 0.0);
        for i in 0..4 {
            for j in 0..4 {
                inverse[i][j] = adj[i][j] / det;
            }
        }
        Self { matrix: inverse }
    }


    pub fn transpose(&self) -> Self {
        let mut transpose = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                transpose[i][j] = self.matrix[j][i];
            }
        }

        Self { matrix: transpose }
    }
}

impl Mul<Vec3> for Matrix4 {
    type Output = Vec3;
    fn mul(self, p: Vec3) -> Vec3 {
        let mut result = Vec3::zero();
        for i in 0..3 {
            result[i] = (self.matrix[i][0] * p.x)
                      + (self.matrix[i][1] * p.y)
                      + (self.matrix[i][2] * p.z)
                      +  self.matrix[i][3];
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_vector_multiply() {
        let matrix = Matrix4::new([
            [2.0, 4.0, 1.0, 3.0],
            [3.0, 5.0, 7.0, 8.0],
            [7.0, 8.0, 9.0, 9.0],
        ]);
        let vector = Vec3::new(23.0, 42.0, 125.0);
        let result = matrix * vector;
        assert_eq!(Vec3::new(342.0, 1162.0, 1631.0), result);
    }

    #[test]
    fn matrix_determinant() {
        let matrix = [
            [1.0, 0.0, 2.0, -1.0],
            [3.0, 0.0, 0.0, 5.0],
            [2.0, 1.0, 4.0, -3.0],
            [1.0, 0.0, 5.0, 0.0],
        ];
        assert_eq!(Matrix4::determinant(matrix, 4), 30.0);
    }

    #[test]
    fn matrix_inverse() {
        let matrix = Matrix4::new([
            [2.0, 4.0, 1.0, 3.0],
            [3.0, 5.0, 7.0, 8.0],
            [7.0, 8.0, 9.0, 9.0],
        ]);
        assert_eq!(matrix.inverse(), Matrix4::new([
            [-1.0 / 5.0, -28.0 / 55.0, 23.0 / 55.0, 10.0 / 11.0],
            [2.0 / 5.0, 1.0 / 5.0, -1.0 / 5.0, -1.0],
            [-1.0 / 5.0, 12.0 / 55.0, -2.0 / 55.0, -9.0 / 11.0],
        ]));
    }

    #[test]
    fn matrix_transpose() {
        let matrix = Matrix4::new([
            [2.0, 4.0, 1.0, 3.0],
            [3.0, 5.0, 7.0, 8.0],
            [7.0, 8.0, 9.0, 9.0],
        ]);
        assert_eq!(matrix.transpose().matrix, [
            [2.0, 3.0, 7.0, 0.0],
            [4.0, 5.0, 8.0, 0.0],
            [1.0, 7.0, 9.0, 0.0],
            [3.0, 8.0, 9.0, 1.0],
        ]);
    }
}
