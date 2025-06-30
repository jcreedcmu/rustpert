use crate::env::Env;
use crate::geom::Point3d;
use crate::geom::Quat;
use crate::interval::Interval;
use rug::Rational;

/// The state of search.
///
/// Describes a 7-d hypercube in parameter space.
pub struct State {
    outer: Point3d<Interval<Rational>>,
    inner: Point3d<Interval<Rational>>,
    translate: Interval<Rational>,
}

pub fn quat_in_in_patch(env: &Env, quat: &Quat<Interval<Rational>>) -> bool {
    for (i, face) in env.faces.iter().enumerate() {
        //
    }
    false
}
