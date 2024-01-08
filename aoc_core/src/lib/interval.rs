use crate::num::Integer;
use std::cmp::{max, min};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Interval<T = i32>
where
    T: Integer,
{
    pub start: T,
    pub end: T,
}

pub struct UnionParts<T>
where
    T: Integer,
{
    pub left: Option<Interval<T>>,
    pub center: Option<Interval<T>>,
    pub right: Option<Interval<T>>,
}

impl<T> TryFrom<(T, T)> for Interval<T>
where
    T: Integer,
{
    type Error = ();

    fn try_from(value: (T, T)) -> Result<Self, Self::Error> {
        if value.0 < value.1 {
            Ok(Self::new(value.0, value.1))
        } else {
            Err(())
        }
    }
}

#[allow(clippy::from_over_into)]
impl<T> Into<(T, T)> for &Interval<T>
where
    T: Integer,
{
    fn into(self) -> (T, T) {
        (self.start, self.end)
    }
}

impl<T> Interval<T>
where
    T: Integer,
{
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> T {
        self.end - self.start
    }

    pub fn union_parts(&self, other: &Self) -> UnionParts<T> {
        let (x0, y0) = self.into();
        let (x1, y1) = other.into();

        let a0 = min(x0, x1);
        let a1 = max(x0, x1);
        let b0 = min(y0, y1);
        let b1 = max(y0, y1);

        let left = Interval::try_from((a0, a1)).ok();
        let center = Interval::try_from((a1, b0)).ok();
        let right = Interval::try_from((b0, b1)).ok();

        UnionParts {
            left,
            center,
            right,
        }
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let (x0, y0) = self.into();
        let (x1, y1) = other.into();
        let (x, y) = (max(x0, x1), min(y0, y1));

        Interval::try_from((x, y)).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let int = Interval::new(1, 10);
        assert_eq!(int.start, 1);
        assert_eq!(int.end, 10);
    }

    #[test]
    fn test_len() {
        let int = Interval::new(1, 10);
        assert_eq!(int.len(), 9);
    }

    #[test]
    fn test_zero() {
        let int = Interval::new(0, 0);
        assert_eq!(int.len(), 0);
    }

    #[test]
    fn test_union_parts() {
        let a = Interval::new(2, 6);
        let b = Interval::new(0, 10);

        let union_parts = a.union_parts(&b);

        assert_eq!(union_parts.left, Some(Interval::new(0, 2)));
        assert_eq!(union_parts.center, Some(Interval::new(2, 6)));
        assert_eq!(union_parts.right, Some(Interval::new(6, 10)));
    }

    #[test]
    fn test_union_parts2() {
        let a = Interval::new(2, 6);
        let b = Interval::new(2, 10);

        let union_parts = a.union_parts(&b);

        assert_eq!(union_parts.left, None);
        assert_eq!(union_parts.center, Some(Interval::new(2, 6)));
        assert_eq!(union_parts.right, Some(Interval::new(6, 10)));
    }
}
