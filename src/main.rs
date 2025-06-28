use std::fs;

mod json_rep {
    use serde;
    use serde::{Deserialize, Deserializer, Serialize};

    fn custom<'de, D>(deserializer: D) -> Result<CustomRational, D::Error>
    where
        D: Deserializer<'de>,
    {
        let r = Rational::deserialize(deserializer)?;
        Ok(CustomRational {
            nume: format!("+{}", r.numerator),
            denom: format!("+{}", r.denominator),
        })
    }

    #[derive(Debug, Serialize)]
    pub struct CustomRational {
        nume: String,
        denom: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Rational {
        #[serde(rename = "n")]
        numerator: String,
        #[serde(rename = "d")]
        denominator: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Vertex {
        #[serde(deserialize_with = "custom")]
        x: CustomRational,
        y: Rational,
        z: Rational,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Polyhedron {
        #[serde(rename = "v")]
        vertices: Vec<Vertex>,
        #[serde(rename = "f")]
        faces: Vec<Vec<u32>>,
    }
}

fn main() -> std::io::Result<()> {
    let json_string = fs::read_to_string("data/rational-snub.json")?;

    let json: json_rep::Polyhedron =
        serde_json::from_str(&json_string).expect("JSON was not well-formatted");

    println!("{}", serde_json::to_string(&json)?);
    Ok(())
}
