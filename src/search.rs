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

/// An environment type containing other things
/// we need to know about for search.
///
/// Doesn't vary as we traverse different hypercubes.
pub struct Env {
    vertices: Vec<Point3d<Rational>>,
    /// List of faces, each face is an oriented cyclic list of vertex
    /// indices indexing into `.vertices`.
    faces: Vec<Vec<usize>>,
    /// List of indexes into `.faces` asserting that these faces should
    /// remain in the positive orientation after rotation. If that constraint
    /// is satisfied, then we're in the chosen "patch".
    /// FIXME: Explain the exact sign convention that "positive orientation" entails.
    /// FIXME(eventually): Generalize to two sets of positive faces for two patches.
    /// (right now we're just testing one patch against itself)
    positive_faces: Vec<usize>,
    /// Cyclic ordered list of indexes into `.vertices` giving a
    /// traversal of the minimal set of vertices supporting the convex
    /// hull under projection.
    ///
    /// This is in principle derivable from the set of positive faces:
    /// The edges lying on the convex hull circuit are exactly the
    /// edges that appear exactly once in positive_faces, and we can
    /// take their orientation in the positive faces in which they
    /// appear to determine the ordering.
    circuit: Vec<usize>,
}
