use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WireRational {
    #[serde(rename = "n")]
    numerator: String,
    #[serde(rename = "d")]
    denominator: String,
}

mod custom_rational {
    use super::WireRational;
    use rug::Assign;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<rug::Rational, D::Error>
    where
        D: Deserializer<'de>,
    {
        let r = WireRational::deserialize(deserializer)?;
        let mut n = rug::Integer::new();
        let mut d = rug::Integer::new();
        n.assign(rug::Integer::parse(r.numerator).unwrap());
        d.assign(rug::Integer::parse(r.denominator).unwrap());
        Ok(rug::Rational::from((n, d)))
    }

    pub fn serialize<S>(r: &rug::Rational, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rr = WireRational {
            numerator: r.numer().to_string(),
            denominator: r.denom().to_string(),
        };
        rr.serialize(serializer)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vertex {
    #[serde(with = "custom_rational")]
    pub x: rug::Rational,
    #[serde(with = "custom_rational")]
    pub y: rug::Rational,
    #[serde(with = "custom_rational")]
    pub z: rug::Rational,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Polyhedron {
    #[serde(rename = "v")]
    pub vertices: Vec<Vertex>,
    #[serde(rename = "f")]
    pub faces: Vec<Vec<u32>>,
}
