mod env;
mod geom;
mod interval;
mod json_rep;
mod render;
mod render_geom;
mod search;

use geom::Point3d;
use rug::Rational;
use std::fs;

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

    // Convert from wire format to Point3d<Rational>
    let vertices: Vec<Point3d<Rational>> =
        vertices.into_iter().map(|v| Point3d { x: v.x, y: v.y, z: v.z }).collect();

    // A rotation
    let q: geom::Quat<rug::Rational> = geom::Quat {
        r: rug::Rational::from(10),
        a: rug::Rational::from((99, 10)),
        b: rug::Rational::from((11, 20)),
        c: rug::Rational::from((20, 10)),
    };

    let env = env::Env::new(vertices, faces, q, vec![0, 16, 3, 23, 5, 14, 6, 19, 9, 20, 2, 17]);

    fs::write("/tmp/a.svg", env.render())?;
    Ok(())
}
