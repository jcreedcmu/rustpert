use crate::env;
use crate::geom::Point3d;
use crate::render_geom::{Point2d, Poly, Xform};

/// Formats a pair of points as svg
fn format_line(p1: &Point2d, p2: &Point2d) -> String {
    format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" style="stroke:black;stroke-width:1" />"#,
        p1.x, p1.y, p2.x, p2.y
    )
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

fn render_faces(xf: &Xform, proj_faces: &Vec<Poly>, face_indexes: &Vec<usize>) -> String {
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

pub fn render(env: &env::Env) -> String {
    let xf = Xform { scale: 200., translate: Point2d { x: 500., y: 500. } };

    let proj_faces = env.get_proj_faces();
    let poly_strs =
        proj_faces.iter().map(|face| format_xf_poly(&xf, face)).collect::<Vec<String>>().join("\n");
    let label_strs = env
        .vertices
        .iter()
        .enumerate()
        .map(|(i, v)| make_label(&xf, i, v))
        .collect::<Vec<String>>()
        .join("\n");
    let face_strs = render_faces(&xf, &proj_faces, &env.positive_faces);
    format!(
        r#"
<svg height="1000" width="1000" xmlns="http://www.w3.org/2000/svg">
{}{}{}
</svg>
"#,
        face_strs, poly_strs, label_strs,
    )
}
