//! 3D Vector module.
//!
//! Provides [`Vector`](struct@Vector) struct.
use crate::error::GeometryErr;
use crate::misc::IsClose;
use crate::normal::Normal;
use crate::point::Point;
use std::fmt;
use std::ops::{Add, Mul, Sub};

/// 3D Vector struct.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector {
    /// x component
    pub x: f32,
    /// y component
    pub y: f32,
    /// z component
    pub z: f32,
}

impl Vector {
    /// Return the reversed vector.
    pub fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Compute the dot product between two vectors.
    pub fn dot(self, other: Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Return the squared norm (Euclidean length) of a vector.
    ///
    /// This is faster than [`norm`](#method.squared_norm) if you just need the squared norm.
    pub fn squared_norm(self) -> f32 {
        self.dot(self)
    }

    /// Return the norm (Euclidean length) of a vector.
    pub fn norm(self) -> f32 {
        f32::sqrt(self.squared_norm())
    }

    /// Modify the vector's norm so that it becomes equal to 1.
    ///
    /// And return the normalized vector inside [`std::result::Result`].\
    /// If the normalization operation wasn't possible result is an
    /// [`GeometryErr`] error variant.
    pub fn normalize(mut self) -> Result<Vector, GeometryErr> {
        if self.norm() > 0_f32 {
            self = self * (1_f32 / self.norm());
            Ok(self)
        } else {
            Err(GeometryErr::UnableToNormalize(self.norm()))
        }
    }
}

impl From<(f32, f32, f32)> for Vector {
    fn from(xyz: (f32, f32, f32)) -> Self {
        Self {
            x: xyz.0,
            y: xyz.1,
            z: xyz.2,
        }
    }
}

impl From<Point> for Vector {
    fn from(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }
}

impl From<Normal> for Vector {
    fn from(normal: Normal) -> Self {
        Self {
            x: normal.x,
            y: normal.y,
            z: normal.z,
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector({}, {}, {})", self.x, self.y, self.z)
    }
}

impl IsClose for Vector {
    /// Return `true` if the three xyz components of two [`Vector`] are [close](trait@IsClose).
    fn is_close(&self, other: Vector) -> bool {
        self.x.is_close(other.x) & self.y.is_close(other.y) & self.z.is_close(other.z)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::misc::EPSILON;

    #[test]
    fn test_is_close() {
        assert!(
            (Vector::from((1.23, 4.56, 7.89)) * Vector::from((9.87, 6.54, 3.21)))
                .is_close(Vector::from((-36.963, 73.926, -36.963)))
        );
        assert!(
            (Vector::from((1.0, 2.0, 3.0)) + Vector::from((1.0, 2.0 + EPSILON * 1e-1, 3.0)))
                .is_close(Vector::from((2.0, 4.0, 6.0)))
        )
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vector::from((1.0, 1.0, 1.0)) + Vector::from((2.0, 2.0, 2.0)),
            Vector::from((3.0, 3.0, 3.0))
        )
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vector::from((1.0, 2.0, 3.0)) - Vector::from((2.0, 2.0, 2.0)),
            Vector::from((-1.0, 0.0, 1.0))
        )
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vector::from((1.0, 1.0, 1.0)).dot(Vector::from((2.0, 1.0, 2.0))),
            5.0
        )
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vector::from((1.0, 1.0, 1.0)) * Vector::from((2.0, 1.0, 2.0)),
            Vector::from((1.0, 0.0, -1.0))
        )
    }

    #[test]
    fn test_mul_scalar() {
        assert_eq!(
            Vector::from((1.0, 1.0, 1.0)) * 2.0,
            Vector::from((2.0, 2.0, 2.0))
        )
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", Vector::from((1.02, -2.00, 1e7))),
            "Vector(1.02, -2, 10000000)"
        );
    }

    #[test]
    fn test_neg() {
        assert_eq!(
            Vector::from((0.0, 0.1, -2.0)).neg(),
            Vector::from((0.0, -0.1, 2.0))
        )
    }

    #[test]
    fn test_squared_norm() {
        assert_eq!(Vector::from((1.0, -2.0, 3.0)).squared_norm(), 14.0)
    }

    #[test]
    fn test_norm() {
        assert_eq!(Vector::from((0.0, -4.0, 3.0)).norm(), 5.0)
    }

    #[test]
    fn test_normalize() {
        let vector = Vector::from((-6. / 7., 2. / 7., -3. / 7.));

        assert!(matches!(
            Vector::from((-6.0, 2.0, -3.0)).normalize(), Ok(v) if v.is_close(vector)
        ));
        assert!(matches!(
            Vector::from((0.0, 0.0, 0.0)).normalize(), Err(GeometryErr::UnableToNormalize(a)) if a == 0_f32
        ))
    }
}