/// A point in 2d
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

/// A polygon in 2d
pub type Poly = Vec<Point2d>;

/// A transformation in 2d that allows scaling and translation
pub struct Xform {
    pub scale: f64,
    pub translate: Point2d,
}

impl Xform {
    /// Applies a 2d transform to a point
    pub fn apply(&self, p: &Point2d) -> Point2d {
        Point2d { x: self.scale * p.x + self.translate.x, y: self.scale * p.y + self.translate.y }
    }
}
