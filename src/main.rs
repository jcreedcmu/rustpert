use std::fs;

mod json_rep {
    use rug::Assign;
    use serde;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    fn custom<'de, D>(deserializer: D) -> Result<CustomRational, D::Error>
    where
        D: Deserializer<'de>,
    {
        let r = Rational::deserialize(deserializer)?;
        let mut n = rug::Integer::new();
        let mut d = rug::Integer::new();
        n.assign(rug::Integer::parse(r.numerator).unwrap());
        d.assign(rug::Integer::parse(r.denominator).unwrap());
        Ok(CustomRational { nume: n, denom: d })
    }

    fn customs<S>(r: &CustomRational, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rr = Rational {
            numerator: r.nume.to_string(),
            denominator: r.denom.to_string(),
        };
        rr.serialize(serializer)
    }

    #[derive(Debug)]
    pub struct CustomRational {
        nume: rug::Integer,
        denom: rug::Integer,
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
        #[serde(serialize_with = "customs")]
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
