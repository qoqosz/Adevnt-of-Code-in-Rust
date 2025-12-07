use crate::num::Integer;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2D<T = i32> {
    pub x: T,
    pub y: T,
}

pub type Point<T = i32> = Point2D<T>;

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Default for Point2D<T>
where
    T: Integer,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

/// Compare with tuple
impl<T> PartialEq<(T, T)> for Point2D<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &(T, T)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

/// Build from tuple
impl<T> From<(T, T)> for Point2D<T> {
    fn from(value: (T, T)) -> Self {
        Point2D {
            x: value.0,
            y: value.1,
        }
    }
}

/// Add two points
impl<T: Add<Output = T>> Add for Point2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// TODO
// impl<'a, T> AddAssign<&'a Point2D<T>> for Point2D<T>
// where
//     T: 'a + DerefMut + Copy + AddAssign<&'a T>,
//     <T as Deref>::Target: AddAssign<T>,
// {
//     fn add_assign(&mut self, rhs: &'a Self) {
//         *self.x += rhs.x;
//         *self.y += rhs.y;
//     }
// }

/// Subtract two points
impl<T: Sub<Output = T>> Sub for Point2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Multiply two points
impl<T: Mul<Output = T>> Mul for Point2D<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

/// Multiply by a scalar
impl<T: Integer + Mul<Output = T>> Mul<T> for Point2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let p = Point::default();
        assert_eq!(p, (0, 0));
    }

    #[test]
    fn test_add() {
        let a = Point::new(2, 3);
        let b = Point::new(5, 6);
        let c = a + b;
        assert_eq!(c, (7, 9));
    }

    #[test]
    fn test_sub() {
        let a = Point::new(2, 3);
        let b = Point::new(5, 6);
        let c = a - b;
        assert_eq!(c, (-3, -3));
    }

    #[test]
    fn test_mul() {
        let a = Point::new(2, 3);
        let b = Point::new(5, 6);
        let c = a * b;
        assert_eq!(c, (10, 18));
    }

    #[test]
    fn test_mul_scalar() {
        let a = Point::new(3, 4);
        let b = a * 2;
        assert_eq!(b, (6, 8));
    }
}
