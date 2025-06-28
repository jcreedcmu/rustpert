use serde;
use serde::{Deserialize, Serialize};

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

mod custom_rational {
    use super::{CustomRational, Rational};
    use rug::Assign;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<CustomRational, D::Error>
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

    pub fn serialize<S>(r: &CustomRational, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rr = Rational {
            numerator: r.nume.to_string(),
            denominator: r.denom.to_string(),
        };
        rr.serialize(serializer)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vertex {
    #[serde(with = "custom_rational")]
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
