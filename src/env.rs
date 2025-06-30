use crate::geom::Point3d;
use crate::render;
use crate::render_geom::{Point2d, Poly};
use rug::Rational;

/// An environment type containing other things
/// we need to know about for search.
///
/// Doesn't vary as we traverse different hypercubes.
pub struct Env {
    pub vertices: Vec<Point3d<Rational>>,
    /// List of faces, each face is an oriented cyclic list of vertex
    /// indices indexing into `.vertices`.
    pub faces: Vec<Vec<usize>>,
    /// List of indexes into `.faces` asserting that these faces should
    /// remain in the positive orientation after rotation. If that constraint
    /// is satisfied, then we're in the chosen "patch".
    /// FIXME: Explain the exact sign convention that "positive orientation" entails.
    /// FIXME(eventually): Generalize to two sets of positive faces for two patches.
    /// (right now we're just testing one patch against itself)
    pub positive_faces: Vec<usize>,
    /// Cyclic ordered list of indexes into `.vertices` giving a
    /// traversal of the minimal set of vertices supporting the convex
    /// hull under projection.
    ///
    /// This is in principle derivable from the set of positive faces:
    /// The edges lying on the convex hull circuit are exactly the
    /// edges that appear exactly once in positive_faces, and we can
    /// take their orientation in the positive faces in which they
    /// appear to determine the ordering.
    pub circuit: Vec<usize>,
}

fn is_positive_face(face_vs: &Vec<Point3d<Rational>>) -> bool {
    let v0 = face_vs[0].clone();
    let v1 = face_vs[1].clone();
    let v2 = face_vs[2].clone();
    ((v1.clone() - v0.clone()).cross(v2 - v0)).z > 0
}

/// Returns the list of indices of faces that have positive orientation
fn get_positive_faces(vs: &Vec<Point3d<Rational>>, fs: &Vec<Vec<usize>>) -> Vec<usize> {
    fs.iter()
        .enumerate()
        .filter_map(|(i, face)| {
            let face_vs: Vec<Point3d<Rational>> = face.iter().map(|fvi| vs[*fvi].clone()).collect();
            if is_positive_face(&face_vs) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

/// Project a 3d vertex into 3d
///
/// The current projection discards the z coordinate.
fn proj_vertex(v: &Point3d<Rational>) -> Point2d {
    Point2d { x: v.x.to_f64(), y: v.y.to_f64() }
}

impl Env {
    pub fn new(
        vertices: Vec<Point3d<Rational>>,
        faces: Vec<Vec<usize>>,
        circuit: Vec<usize>,
    ) -> Env {
        let positive_faces = get_positive_faces(&vertices, &faces);
        Env { vertices, faces, circuit, positive_faces }
    }

    /// Returns svg string
    pub fn render(&self) -> String {
        render::render(&self)
    }

    /// Get the coordinates of all faces of a polyhedron, projected to 2d
    pub fn get_proj_faces(&self) -> Vec<Poly> {
        self.faces
            .iter()
            .map(|face| face.iter().map(|v_ix| proj_vertex(&self.vertices[*v_ix])).collect())
            .collect()
    }
}
