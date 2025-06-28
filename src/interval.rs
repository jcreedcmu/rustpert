#![allow(dead_code)]

use std::ops;

/// An interval between two values
///
/// Represents the knowledge that a value is >= min and <= max.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval<T>
where
    T: Clone,
{
    min: T,
    max: T,
}

impl<T> Interval<T>
where
    T: Clone,
{
    pub fn new(min: T, max: T) -> Interval<T> {
        Interval { min, max }
    }

    pub fn exact(val: T) -> Interval<T> {
        let max = val.clone();
        Interval { min: val, max }
    }
}

// Note that we don't implement all of num_traits::Signed for
// Interval, because I don't think an implementation of signum would
// be meaningful. We don't have trichotomy for intervals. An interval
// spanning zero fails to be positive, and fails to be negative.

impl<T> Interval<T>
where
    T: num_traits::Signed + Clone,
{
    pub fn is_positive(self) -> bool {
        self.min.is_positive()
    }

    pub fn is_negative(self) -> bool {
        self.max.is_negative()
    }
}

impl<T> Interval<T>
where
    T: num_traits::Signed + Ord + Clone,
{
    pub fn square(self) -> Interval<T> {
        let sa = self.min.abs();
        let sac = sa.clone();
        let ba = self.max.abs();
        let bac = ba.clone();
        let sas = sa * sac;
        let sasc = sas.clone();
        let bas = ba * bac;
        let basc = bas.clone();
        Interval {
            min: if self.min.signum() != self.max.signum() {
                T::zero()
            } else {
                sas.min(bas)
            },
            max: basc.max(sasc),
        }
    }
}

impl<T> ops::Add<Interval<T>> for Interval<T>
where
    T: ops::Add<T, Output = T> + Clone,
{
    type Output = Interval<T>;

    fn add(self, rhs: Interval<T>) -> Interval<T> {
        Interval {
            min: self.min + rhs.min,
            max: self.max + rhs.max,
        }
    }
}

impl<T> ops::Mul<Interval<T>> for Interval<T>
where
    T: ops::Mul<T, Output = T> + Clone + Ord,
{
    type Output = Interval<T>;

    fn mul(self, rhs: Interval<T>) -> Interval<T> {
        let rsc = rhs.min.clone();
        let ssc = self.min.clone();
        let rbc = rhs.max.clone();
        let sbc = self.max.clone();

        let bs = self.max * rhs.min;
        let ss = self.min * rsc;
        let sb = ssc * rhs.max;
        let bb = sbc * rbc;

        let bsc = bs.clone();
        let ssc = ss.clone();
        let sbc = sb.clone();
        let bbc = bb.clone();
        Interval {
            min: bs.min(ss).min(sb).min(bb),
            max: bsc.max(ssc).max(sbc).max(bbc),
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

    #[test]
    fn test_rational() {
        let half = Interval::new(rug::Rational::from((1, 4)), rug::Rational::from((3, 4)));
        let three_halves = Interval::new(rug::Rational::from((5, 4)), rug::Rational::from((7, 4)));
        assert_eq!(
            half + three_halves,
            Interval::new(rug::Rational::from((3, 2)), rug::Rational::from((5, 2)))
        );
    }
}
