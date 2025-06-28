use std::fs;
mod json_rep;

fn main() -> std::io::Result<()> {
    let json_string = fs::read_to_string("data/rational-snub.json")?;

    let poly: json_rep::Polyhedron =
        serde_json::from_str(&json_string).expect("JSON was not well-formatted");

    println!("JSON:");
    println!("=========");
    println!("{}", serde_json::to_string(&poly)?);
    println!("Vertices:");
    println!("=========");
    for v in poly.vertices {
        println!(
            "{{x: {}, y: {}, z: {}}}",
            v.x.to_f64(),
            v.y.to_f64(),
            v.z.to_f64(),
        );
    }
    Ok(())
}
