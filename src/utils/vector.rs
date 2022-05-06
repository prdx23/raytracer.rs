use std::ops::{
    Add, Sub, Neg, Mul, Div,
    AddAssign, SubAssign, MulAssign, DivAssign,
    Index, IndexMut,
};

use rand::Rng;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn inf() -> Vec3 {
        Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY)
    }

    pub fn neg_inf() -> Vec3 {
        Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY)
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    pub fn len(&self) -> f64 {
        self.sq_len().sqrt()
    }

    pub fn sq_len(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(&self) -> Self {
        *self / self.len()
    }

    pub fn near_zero(&self) -> bool {
        let e = 1e-8;
        self.x.abs() < e && self.y.abs() < e && self.z.abs() < e
    }

    pub fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
        vector - 2.0 * vector.dot(normal) * normal
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-uv).dot(n).min(1.0);
        let r_perp = etai_over_etat * (uv + cos_theta * n);
        let r_parallel = -(1.0 - r_perp.sq_len()).abs().sqrt() * n;

        r_perp + r_parallel
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let point = Vec3::random(-1.0, 1.0);
            if point.sq_len() >= 1.0 { continue; }
            return point
        }
    }

    pub fn random_in_unit_disc() -> Vec3 {
        loop {
            let rnd = Vec3::random(-1.0, 1.0);
            let point = Vec3::new(rnd.x, rnd.y, 0.0);
            if point.sq_len() >= 1.0 { continue; }
            return point
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere
        } else {
            return -in_unit_sphere
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other, y: self.y * other, z: self.z * other
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other, y: self.y * other, z: self.z * other
        };
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x, y: self * other.y, z: self * other.z
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> Self {
        Self {
            x: self.x * other.x, y: self.y * other.y, z: self.z * other.z
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x * other.x, y: self.y * other.y, z: self.z * other.z
        };
    }
}


impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other, y: self.y / other, z: self.z / other
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x / other, y: self.y / other, z: self.z / other
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        debug_assert!(i < 3);
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
            // _ => panic!("Out of bounds for vector"),
        }
    }
}


impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        debug_assert!(i < 3);
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
            // _ => panic!("Out of bounds for vector"),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vector() {
        let vector = Vec3::new(-32.0, 48.0, 100079.123);
        assert_eq!(vector.x, -32.0);
        assert_eq!(vector.y, 48.0);
        assert_eq!(vector.z, 100079.123);
    }

    #[test]
    fn zero_vector() {
        let vector = Vec3::zero();
        assert_eq!(vector.x, 0.0);
        assert_eq!(vector.y, 0.0);
        assert_eq!(vector.z, 0.0);
    }

    #[test]
    fn vector_add() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:5.0, y:7.0, z:9.0 }, vector1 + vector2);
    }

    #[test]
    fn vector_add_assign() {
        let mut vector1 = Vec3::new(1.0, 2.0, 3.0);
        vector1 += Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:5.0, y:7.0, z:9.0 }, vector1);
    }

    #[test]
    fn vector_sub() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:3.0, y:3.0, z:3.0 }, vector2 - vector1);
    }

    #[test]
    fn vector_sub_assign() {
        let mut vector1 = Vec3::new(4.0, 5.0, 6.0);
        vector1 -= Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3 { x:3.0, y:3.0, z:3.0 }, vector1);
    }

    #[test]
    fn vector_neg() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3 { x:-1.0, y:-2.0, z:-3.0 }, -vector1);
    }

    #[test]
    fn vector_mul() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3 { x:2.0, y:4.0, z:6.0 }, vector1 * 2.0);
    }

    #[test]
    fn vector_mul_assign() {
        let mut vector1 = Vec3::new(1.0, 2.0, 3.0);
        vector1 *= 2.0;
        assert_eq!(Vec3 { x:2.0, y:4.0, z:6.0 }, vector1);
    }

    #[test]
    fn vector_mul_rev() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3 { x:2.0, y:4.0, z:6.0 }, 2.0 * vector1);
    }

    #[test]
    fn vector_mul_vector() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:4.0, y:10.0, z:18.0 }, vector1 * vector2);
    }

    #[test]
    fn vector_mul_assign_vector() {
        let mut vector1 = Vec3::new(1.0, 2.0, 3.0);
        vector1 *= Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:4.0, y:10.0, z:18.0 }, vector1);
    }

    #[test]
    fn vector_div() {
        let vector1 = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(Vec3 { x:1.0, y:2.0, z:3.0 }, vector1 / 2.0);
    }

    #[test]
    fn vector_div_assign() {
        let mut vector1 = Vec3::new(2.0, 4.0, 6.0);
        vector1 /= 2.0;
        assert_eq!(Vec3 { x:1.0, y:2.0, z:3.0 }, vector1);
    }

    #[test]
    fn vector_dot() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(32.0, vector1.dot(vector2));
    }

    #[test]
    fn vector_cross() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:-3.0, y:6.0, z:-3.0 }, vector1.cross(vector2));
    }

    #[test]
    fn vector_len() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(14.0, vector1.sq_len());
        assert_eq!((14.0 as f64).sqrt(), vector1.len());
    }

    // needs float compare with <= e
    // #[test]
    // fn vector_unit() {
    //     let vector1 = Vec3::new(1.0, 2.0, 3.0);
    //     assert_eq!(Vec3 { x:2.0, y:4.0, z:6.0 }, vector1.unit());
    // }

    #[test]
    fn vector_index() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, vector1[0]);
        assert_eq!(2.0, vector1[1]);
        assert_eq!(3.0, vector1[2]);
    }

    #[test]
    #[should_panic(expected = "Out of bounds for vector")]
    fn vector_index_oob() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, vector1[4]);
    }

    #[test]
    fn vector_index_mut() {
        let mut vector1 = Vec3::new(1.0, 2.0, 3.0);
        vector1[0] += 1.0;
        vector1[1] += 1.0;
        vector1[2] += 1.0;
        assert_eq!(2.0, vector1[0]);
        assert_eq!(3.0, vector1[1]);
        assert_eq!(4.0, vector1[2]);
    }

    #[test]
    #[should_panic(expected = "Out of bounds for vector")]
    fn vector_index_mut_oob() {
        let mut vector1 = Vec3::new(1.0, 2.0, 3.0);
        vector1[4] += 1.0;
        assert_eq!(1.0, vector1[1]);
    }
}
