#![allow(dead_code)]

use std::ops;

#[derive(Clone)]
pub struct Quat<T> {
    pub r: T,
    pub a: T,
    pub b: T,
    pub c: T,
}

/// A point in 3d
#[derive(Clone)]
pub struct Point3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Quat<T>
where
    T: ops::Neg<Output = T> + Clone,
{
    pub fn conj(&self) -> Quat<T> {
        Quat {
            r: self.r.clone(),
            a: -self.a.clone(),
            b: -self.b.clone(),
            c: -self.c.clone(),
        }
    }
}

impl<T> Quat<T>
where
    T: ops::Add<T, Output = T> + ops::Mul<T, Output = T> + Clone,
{
    pub fn sqnorm(&self) -> T {
        self.r.clone() * self.r.clone()
            + self.a.clone() * self.a.clone()
            + self.b.clone() * self.b.clone()
            + self.c.clone() * self.c.clone()
    }
}

impl<T> ops::Mul<Quat<T>> for Quat<T>
where
    T: ops::Add<T, Output = T> + ops::Sub<T, Output = T> + ops::Mul<T, Output = T> + Clone,
{
    type Output = Quat<T>;
    fn mul(self, rhs: Quat<T>) -> Quat<T> {
        let Quat {
            r: xr,
            a: xa,
            b: xb,
            c: xc,
        } = self;
        let Quat {
            r: yr,
            a: ya,
            b: yb,
            c: yc,
        } = rhs;
        Quat {
            r: xr.clone() * yr.clone()
                - xa.clone() * ya.clone()
                - xb.clone() * yb.clone()
                - xc.clone() * yc.clone(),
            a: xr.clone() * ya.clone() + xb.clone() * yc.clone() + xa.clone() * yr.clone()
                - xc.clone() * yb.clone(),
            b: xr.clone() * yb.clone() + xc.clone() * ya.clone() + xb.clone() * yr.clone()
                - xa.clone() * yc.clone(),
            c: xr.clone() * yc.clone() + xa.clone() * yb.clone() + xc.clone() * yr.clone()
                - xb.clone() * ya.clone(),
        }
    }
}

impl<T> ops::Mul<Point3d<T>> for Quat<T>
where
    T: ops::Add<T, Output = T>
        + ops::Sub<T, Output = T>
        + ops::Mul<T, Output = T>
        + ops::Div<T, Output = T>
        + ops::Neg<Output = T>
        + num_traits::Zero
        + Clone,
{
    type Output = Point3d<T>;
    fn mul(self, rhs: Point3d<T>) -> Point3d<T> {
        let rhsq = Quat {
            r: T::zero(),
            a: rhs.x,
            b: rhs.y,
            c: rhs.z,
        };
        let sn = self.clone().sqnorm();
        let non_norm = self.clone() * rhsq * self.conj();
        Point3d {
            x: non_norm.a / sn.clone(),
            y: non_norm.b / sn.clone(),
            z: non_norm.c / sn.clone(),
        }
    }
}

impl<T> ops::Sub<Point3d<T>> for Point3d<T>
where
    T: ops::Sub<T, Output = T>,
{
    type Output = Point3d<T>;
    fn sub(self, rhs: Point3d<T>) -> Point3d<T> {
        Point3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Point3d<T>
where
    T: ops::Sub<T, Output = T> + ops::Mul<T, Output = T> + Clone,
{
    pub fn cross(self, rhs: Point3d<T>) -> Point3d<T> {
        Point3d {
            x: self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
            y: self.z.clone() * rhs.x.clone() - self.x.clone() * rhs.z.clone(),
            z: self.x.clone() * rhs.y.clone() - self.y.clone() * rhs.z.clone(),
        }
    }
}
