use std::fs;
mod json_rep;

/// A point in 2d
struct Point {
    x: f64,
    y: f64,
}

/// A polygon in 2d
type Poly = Vec<Point>;

/// A transformation in 2d that allows scaling and translation
struct Xform {
    scale: f64,
    translate: Point,
}

/// Formats a pair of points as svg
fn format_line(p1: &Point, p2: &Point) -> String {
    format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" style="stroke:black;stroke-width:1" />"#,
        p1.x, p1.y, p2.x, p2.y
    )
}

/// Applies a 2d transform to a point
fn apply_xf(xf: &Xform, p: &Point) -> Point {
    Point {
        x: xf.scale * p.x + xf.translate.x,
        y: xf.scale * p.y + xf.translate.y,
    }
}

/// Formats a transformed line as svg
fn format_xf_line(xf: &Xform, p1: &Point, p2: &Point) -> String {
    format_line(&apply_xf(xf, p1), &apply_xf(xf, p2))
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

/// Get the coordinates of all faces of a polyhedron, projected to 2d
///
/// The current projection discards the z coordinate.
fn get_faces(p: &json_rep::Polyhedron) -> Vec<Poly> {
    let mut v: Vec<Poly> = Vec::new();
    for face in p.faces.iter() {
        let mut ps: Vec<Point> = Vec::new();
        for v_ix in face.iter() {
            let q: usize = *v_ix as usize;
            ps.push(Point {
                x: p.vertices[q].x.to_f64(),
                y: p.vertices[q].y.to_f64(),
            });
        }
        v.push(ps);
    }
    v
}

/// Print out some debugging information.
fn main() -> std::io::Result<()> {
    let json_string = fs::read_to_string("data/rational-snub.json")?;

    let poly: json_rep::Polyhedron =
        serde_json::from_str(&json_string).expect("JSON was not well-formatted");

    println!("JSON:");
    println!("=========");
    println!("{}", serde_json::to_string(&poly)?);
    println!("Vertices:");
    println!("=========");
    for v in &poly.vertices {
        println!(
            "{{x: {}, y: {}, z: {}}}",
            v.x.to_f64(),
            v.y.to_f64(),
            v.z.to_f64(),
        );
    }
    let xf = Xform {
        scale: 75.,
        translate: Point { x: 250., y: 250. },
    };
    //    let lines_str = format_xf_line(&xf, &Point { x: 0., y: 0. }, &Point { x: 1., y: 1. });
    let faces = get_faces(&poly);
    let poly_strs = faces
        .iter()
        .map(|face| format_xf_poly(&xf, face))
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(
        "/tmp/a.svg",
        format!(
            r#"
<svg height="500" width="500" xmlns="http://www.w3.org/2000/svg">
{}
</svg>
"#,
            poly_strs
        ),
    )?;
    Ok(())
}
