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
    //// To get this to work, I need to figure out how intervals behave with normalization.
    //// The good news is that all the quaternions I'm dealing with are going to have
    //// squared norm strictly away from zero. They're all 1 + aî + bĵ + ck̂. So even if
    //// a, b, c are intervals, maybe even containing zero, the inverse squared norm
    //// is going to be ≤ 1.
    // let rotated_vertices = crate::geom::rotate_vertices(quat, &env.vertices);

    for (i, face) in env.faces.iter().enumerate() {
        //
    }
    false
}
