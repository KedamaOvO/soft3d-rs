use std::ops::{Add, Sub, Mul};
use std::fmt;
use std::fmt::{Formatter, Error};

#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IVector {}

impl<'a, 'b> Add<&'a Vector> for &'b Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: &'a Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<'a, 'b> Sub<&'a Vector> for &'a Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: &'a Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<'a, 'b> Mul<&'a Vector> for &'b Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: &'a Vector) -> Self::Output {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl Vector {
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector {
            x,
            y,
            z,
            w,
        }
    }

    #[inline]
    pub const fn point(x: f32, y: f32, z: f32) -> Self {
        Vector {
            x,
            y,
            z,
            w: 1.0,
        }
    }

    #[inline]
    pub const fn vec(x: f32, y: f32, z: f32) -> Self {
        Vector {
            x,
            y,
            z,
            w: 0.0,
        }
    }

    #[inline]
    pub const fn vec2(x: f32, y: f32,) ->Self{
        Vector {
            x,
            y,
            z: 0.0,
            w: 0.0,
        }
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> Vector {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: 0f32,
        }
    }

    #[inline]
    pub fn scale(&self, rhs: f32) -> Vector {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let len_inv = 1.0f32 / self.length();
        self.scale(len_inv)
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn lerp(a: &Vector, b: &Vector, t: f32) -> Vector {
        a + &(b - a).scale(t)
    }
}


impl fmt::Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[{},{},{},{}]", self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;

    #[test]
    fn cross_test() {
        let a = Vector::new(1.0, 2.0, 3.0, 0.0);
        let b = Vector::new(4.0, 5.0, 6.0, 0.0);
        assert_eq!(Vector::new(-3.0, 6.0, -3.0, 0.0), a.cross(&b))
    }

    #[test]
    fn length_test() {
        let a = Vector::new(1.0, 1.0, 1.0, 0.0);
        assert_eq!(3f32.sqrt(), a.length())
    }
}