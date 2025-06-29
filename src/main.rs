use std::fs;
mod geom;
mod interval;
mod json_rep;
use geom::Point3d;
use rug::Rational;

/// A point in 2d
struct Point2d {
    x: f64,
    y: f64,
}

/// A polygon in 2d
type Poly = Vec<Point2d>;

/// A transformation in 2d that allows scaling and translation
struct Xform {
    scale: f64,
    translate: Point2d,
}

/// Formats a pair of points as svg
fn format_line(p1: &Point2d, p2: &Point2d) -> String {
    format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" style="stroke:black;stroke-width:1" />"#,
        p1.x, p1.y, p2.x, p2.y
    )
}

impl Xform {
    /// Applies a 2d transform to a point
    fn apply(&self, p: &Point2d) -> Point2d {
        Point2d { x: self.scale * p.x + self.translate.x, y: self.scale * p.y + self.translate.y }
    }
}

/// Formats a transformed line as svg
fn format_xf_line(xf: &Xform, p1: &Point2d, p2: &Point2d) -> String {
    format_line(&xf.apply(p1), &xf.apply(p2))
}

/// Formats a transformed polygon as svg
fn format_xf_poly(xf: &Xform, p: &Poly) -> String {
    let mut s: String = "".to_string();
    let len = p.len();
    for i in 0..len {
        s.push_str(&format_xf_line(xf, &p[i], &p[(i + 1) % len]));
    }
    s
}

/// Project a 3d vertex into 3d
///
/// The current projection discards the z coordinate.
fn proj_vertex(v: &Point3d<Rational>) -> Point2d {
    Point2d { x: v.x.to_f64(), y: v.y.to_f64() }
}

/// Get the coordinates of all faces of a polyhedron, projected to 2d
fn get_proj_faces(vs: &Vec<Point3d<Rational>>, fs: &Vec<Vec<u32>>) -> Vec<Poly> {
    fs.iter()
        .map(|face| face.iter().map(|v_ix| proj_vertex(&vs[*v_ix as usize])).collect())
        .collect()
}

fn render_faces(xf: &Xform, proj_faces: &Vec<Poly>, face_indexes: Vec<usize>) -> String {
    face_indexes
        .iter()
        .map(|face_index| {
            let points_str = proj_faces[*face_index]
                .iter()
                .map(|p| {
                    let q = xf.apply(p);
                    format!("{},{}", q.x, q.y)
                })
                .collect::<Vec<String>>()
                .join(" ");
            format!(r#" <polygon points="{}" style="fill:#04f;fill-opacity:0.4;" /> "#, points_str)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn make_label(xf: &Xform, i: usize, v: &Point3d<rug::Rational>) -> String {
    let c = xf.apply(&Point2d { x: v.x.to_f64(), y: v.y.to_f64() });
    let label_scale: f64 = 1.075;
    let d = xf.apply(&Point2d { x: label_scale * v.x.to_f64(), y: label_scale * v.y.to_f64() });

    format!(
        r#"
<circle cx="{}" cy="{}" r="4" fill="black"  />
<circle cx="{}" cy="{}" r="9" fill="white"  />
    <text font-family="iosevka" font-weight="bold" font-size="12" text-anchor="middle" dominant-baseline="middle" x="{}" y="{}" >{}</text>"#,
        c.x,
        c.y,
        d.x,
        d.y,
        d.x,
        d.y + 1., // slight adjustment to position text better
        i
    )
}

/// Returns the list of indices of faces that have positive orientation
fn get_positive_faces(vs: &Vec<Point3d<Rational>>, fs: &Vec<Vec<u32>>) -> Vec<usize> {
    fs.iter()
        .enumerate()
        .filter_map(|(i, face)| {
            let v0 = vs[face[0] as usize].clone();
            let v1 = vs[face[1] as usize].clone();
            let v2 = vs[face[2] as usize].clone();
            if i == 24 {
                println!("hello {:?} {:?} {:?}", v0.clone(), v1.clone(), v2.clone());
            }
            let cprod = (v1.clone() - v0.clone()).cross(v2 - v0);
            if i == 24 {
                println!("hello {:?} {:?} ", cprod.z, cprod.z.to_f64());
            }
            if cprod.z > 0 {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

/// Print out some debugging information.
fn main() -> std::io::Result<()> {
    let json_string = fs::read_to_string("data/rational-snub.json")?;

    let poly: json_rep::Polyhedron =
        serde_json::from_str(&json_string).expect("JSON was not well-formatted");

    println!("JSON:");
    println!("=========");
    println!("{}", serde_json::to_string(&poly)?);

    let json_rep::Polyhedron { vertices, faces } = poly;

    println!("Vertices:");
    println!("=========");
    for v in &vertices {
        println!("{{x: {}, y: {}, z: {}}}", v.x.to_f64(), v.y.to_f64(), v.z.to_f64(),);
    }
    let xf = Xform { scale: 200., translate: Point2d { x: 500., y: 500. } };

    // Convert from wire format to Point3d<Rational>
    let vertices: Vec<Point3d<Rational>> =
        vertices.into_iter().map(|v| Point3d { x: v.x, y: v.y, z: v.z }).collect();

    // Apply a rotation
    let q: geom::Quat<rug::Rational> = geom::Quat {
        r: rug::Rational::from(10),
        a: rug::Rational::from((99, 10)),
        b: rug::Rational::from((11, 20)),
        c: rug::Rational::from((20, 10)),
    };

    let vertices = vertices.into_iter().map(|v| q.clone() * v).collect();

    let positive_faces = get_positive_faces(&vertices, &faces);
    let proj_faces = get_proj_faces(&vertices, &faces);
    let poly_strs =
        proj_faces.iter().map(|face| format_xf_poly(&xf, face)).collect::<Vec<String>>().join("\n");
    let label_strs = vertices
        .iter()
        .enumerate()
        .map(|(i, v)| make_label(&xf, i, v))
        .collect::<Vec<String>>()
        .join("\n");
    let face_strs = render_faces(&xf, &proj_faces, positive_faces);
    fs::write(
        "/tmp/a.svg",
        format!(
            r#"
<svg height="1000" width="1000" xmlns="http://www.w3.org/2000/svg">
{}{}{}
</svg>
"#,
            face_strs, poly_strs, label_strs,
        ),
    )?;
    Ok(())
}
