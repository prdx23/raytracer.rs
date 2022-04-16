use std::ops::{Add, Sub, Neg, Mul, Div};


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

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self { x: self.x / other, y: self.y / other, z: self.z / other }
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
    fn vector_sub() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3 { x:3.0, y:3.0, z:3.0 }, vector2 - vector1);
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
    fn vector_div() {
        let vector1 = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(Vec3 { x:1.0, y:2.0, z:3.0 }, vector1 / 2.0);
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

    // #[test]
    // fn vector_unit() {
    //     let vector1 = Vec3::new(1.0, 2.0, 3.0);
    //     assert_eq!(Vec3 { x:2.0, y:4.0, z:6.0 }, vector1.unit());
    // }
}
