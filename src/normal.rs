//! 3D Normal module.
//!
//! Provides [`Normal`](struct@Normal) struct.
use crate::error::GeometryErr;
use crate::misc::IsClose;
use crate::vector::Vector;
use std::fmt;
use std::ops::Mul;

/// 3D Normal struct.
///
/// **Note:** a 3D normal is a 3D vector with norm equal to 1, but
/// it acts differently when 3D [`transformation`](../transformation) is applied.\
/// So is better to create 2 two different objects (structs) despite similarity
/// and doubling the code.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Normal {
    /// x component.
    pub x: f32,
    /// y component.
    pub y: f32,
    /// z component.
    pub z: f32,
}

impl Normal {
    /// Return the reversed normal.
    pub fn neg(&self) -> Normal {
        Normal {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Compute the dot product between two normals.
    pub fn dot(self, other: Normal) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Return the squared norm (Euclidean length) of a normal.
    ///
    /// This is faster than [`norm`](#method.squared_norm) if you just need the squared norm.
    pub fn squared_norm(self) -> f32 {
        self.dot(self)
    }

    /// Return the norm (Euclidean length) of a normal.
    pub fn norm(self) -> f32 {
        f32::sqrt(self.squared_norm())
    }

    /// Modify the normal's norm so that it becomes equal to 1.
    ///
    /// And return the normalized normal inside [`std::result::Result`].\
    /// If the normalization operation wasn't possible result is an
    /// [`GeometryErr`] error variant.
    pub fn normalize(mut self) -> Result<Normal, GeometryErr> {
        if self.norm() > 0_f32 {
            self = self * (1_f32 / self.norm());
            Ok(self)
        } else {
            Err(GeometryErr::UnableToNormalize(self.norm()))
        }
    }
}

impl From<(f32, f32, f32)> for Normal {
    fn from(xyz: (f32, f32, f32)) -> Self {
        Self {
            x: xyz.0,
            y: xyz.1,
            z: xyz.2,
        }
    }
}

impl fmt::Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Normal({}, {}, {})", self.x, self.y, self.z)
    }
}

impl IsClose for Normal {
    /// Return `true` if the three xyz components of two [`Normal`] are [close](trait@IsClose).
    fn is_close(&self, other: Normal) -> bool {
        self.x.is_close(other.x) & self.y.is_close(other.y) & self.z.is_close(other.z)
    }
}

impl Mul<Vector> for Normal {
    type Output = Normal;

    fn mul(self, rhs: Vector) -> Self::Output {
        Normal {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Mul<Normal> for Normal {
    type Output = Normal;

    fn mul(self, rhs: Normal) -> Self::Output {
        Normal {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Mul<f32> for Normal {
    type Output = Normal;

    fn mul(self, rhs: f32) -> Self::Output {
        Normal {
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
            Normal::from((1.0, 2.0 + EPSILON * 1e-1, 3.0)).is_close(Normal::from((1.0, 2.0, 3.0)))
        );
        assert!(!Normal::from((2.0, 1.0 + EPSILON, 3.0)).is_close(Normal::from((2.0, 1.0, 3.0))))
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Normal::from((1.0, 1.0, 1.0)).dot(Normal::from((2.0, 1.0, 2.0))),
            5.0
        )
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Normal::from((1.0, 1.0, 1.0)) * Normal::from((2.0, 1.0, 2.0)),
            Normal::from((1.0, 0.0, -1.0))
        );
        assert_eq!(
            Normal::from((1.0, 1.0, 1.0)) * Vector::from((2.0, 1.0, 2.0)),
            Normal::from((1.0, 0.0, -1.0))
        )
    }

    #[test]
    fn test_mul_scalar() {
        assert_eq!(
            Normal::from((1.0, 1.0, 1.0)) * 2.0,
            Normal::from((2.0, 2.0, 2.0))
        )
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", Normal::from((1.02, -2.00, 1e7))),
            "Normal(1.02, -2, 10000000)"
        );
    }

    #[test]
    fn test_neg() {
        assert_eq!(
            Normal::from((0.0, 0.1, -2.0)).neg(),
            Normal::from((0.0, -0.1, 2.0))
        )
    }

    #[test]
    fn test_squared_norm() {
        assert_eq!(Normal::from((1.0, -2.0, 3.0)).squared_norm(), 14.0)
    }

    #[test]
    fn test_norm() {
        assert_eq!(Normal::from((0.0, -4.0, 3.0)).norm(), 5.0)
    }

    #[test]
    fn test_normalize() {
        let normal = Normal::from((2. / 7., 6. / 7., 3. / 7.));

        assert!(matches!(
            Normal::from((4.0, 12.0, 6.0)).normalize(), Ok(v) if v.is_close(normal)
        ));
        assert!(matches!(
            Normal::from((0.0, 0.0, 0.0)).normalize(), Err(GeometryErr::UnableToNormalize(a)) if a == 0_f32
        ))
    }
}
