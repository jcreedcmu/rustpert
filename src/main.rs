use std::fs;
mod json_rep;

fn main() -> std::io::Result<()> {
    let json_string = fs::read_to_string("data/rational-snub.json")?;

    let json: json_rep::Polyhedron =
        serde_json::from_str(&json_string).expect("JSON was not well-formatted");

    println!("{}", serde_json::to_string(&json)?);
    Ok(())
}
