#![allow(dead_code)]

use std::ops;

/// An interval between two values
///
/// Represents the knowledge that a value is >= min and <= max.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval<T>
where
    T: Copy,
{
    min: T,
    max: T,
}

impl<T> Interval<T>
where
    T: Copy,
{
    pub fn new(min: T, max: T) -> Interval<T> {
        Interval { min, max }
    }

    pub fn exact(val: T) -> Interval<T> {
        Interval { min: val, max: val }
    }
}

impl<T> Interval<T>
where
    T: num_traits::Signed + Ord + Copy,
{
    pub fn square(self) -> Interval<T> {
        let sa = self.min.abs();
        let ba = self.max.abs();
        let sas = sa * sa;
        let bas = ba * ba;
        Interval {
            min: if self.min.signum() != self.max.signum() {
                T::zero()
            } else {
                sas.min(bas)
            },
            max: bas.max(sas),
        }
    }
}

impl ops::Add<Interval<i32>> for Interval<i32> {
    type Output = Interval<i32>;

    fn add(self, rhs: Interval<i32>) -> Interval<i32> {
        Interval {
            min: self.min + rhs.min,
            max: self.max + rhs.max,
        }
    }
}

impl ops::Mul<Interval<i32>> for Interval<i32> {
    type Output = Interval<i32>;

    fn mul(self, rhs: Interval<i32>) -> Interval<i32> {
        let bs = self.max * rhs.min;
        let ss = self.min * rhs.min;
        let sb = self.min * rhs.max;
        let bb = self.max * rhs.max;

        Interval {
            min: bs.min(ss).min(sb).min(bb),
            max: bs.max(ss).max(sb).max(bb),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            Interval::new(0, 4) + Interval::new(1, 10),
            Interval::new(1, 14)
        );
    }
    #[test]
    fn test_mul1() {
        assert_eq!(
            Interval::new(-3, 4) * Interval::new(-100, 10),
            Interval::new(-400, 300)
        );
    }
    #[test]
    fn test_mul2() {
        assert_eq!(
            Interval::new(3, 4) * Interval::new(-100, 10),
            Interval::new(-400, 40)
        );
    }
    #[test]
    fn test_mul3() {
        assert_eq!(
            Interval::new(-4, 3) * Interval::new(-100, 10),
            Interval::new(-300, 400)
        );
    }
    #[test]
    fn test_mul4() {
        assert_eq!(
            Interval::new(-4, -3) * Interval::new(-100, 10),
            Interval::new(-40, 400)
        );
    }
    #[test]
    fn test_square1() {
        assert_eq!((Interval::new(-1, -8)).square(), Interval::new(1, 64));
    }
    #[test]
    fn test_square2() {
        assert_eq!((Interval::new(-1, 8)).square(), Interval::new(0, 64));
    }
    #[test]
    fn test_square3() {
        assert_eq!((Interval::new(1, 8)).square(), Interval::new(1, 64));
    }

    // Calling .square() sometimes yields a tighter interval than
    // simply self-multiplying
    #[test]
    fn test_square4() {
        let iv = Interval::new(1, -8);
        assert_eq!(iv.square(), Interval::new(0, 64));
        assert_eq!(iv * iv, Interval::new(-8, 64));
    }
}
