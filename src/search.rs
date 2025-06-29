use crate::geom::Point3d;
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
