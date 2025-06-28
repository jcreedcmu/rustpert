#![allow(dead_code)]

use std::ops;

pub struct Quat<T> {
    r: T,
    a: T,
    b: T,
    c: T,
}

/// A point in 3d
struct Point3d<T> {
    x: T,
    y: T,
    z: T,
}

// impl<T> ops::Mul<Quat<T>> for Quat<T>
// where
//     T: ops::Add<T, Output = T> + ops::Mul<T, Output = T> + Clone,
// {
//     type Output = Quat<T>;
//     fn mul(self, rhs: Quat<T>) -> Quat<T> {
//         //
//     }
// }
