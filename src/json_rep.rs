use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// A rational number in the format of the json that tom7 gave me.
///
/// "n" for numerator and "d" for denominator, as decimal integer
/// strings.
pub struct WireRational {
    #[serde(rename = "n")]
    numerator: String,
    #[serde(rename = "d")]
    denominator: String,
}

/// Serialization adapter so that raw num/denom strings in json become
/// actual GMP rationals at runtime.
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

/// A vertex of a polyhedron
#[derive(Serialize, Deserialize, Debug)]
pub struct Vertex {
    #[serde(with = "custom_rational")]
    pub x: rug::Rational,
    #[serde(with = "custom_rational")]
    pub y: rug::Rational,
    #[serde(with = "custom_rational")]
    pub z: rug::Rational,
}

/// A polyhedron with vertex and face information
#[derive(Serialize, Deserialize, Debug)]
pub struct Polyhedron {
    #[serde(rename = "v")]
    /// The vertices, represented as triples of rational coordinates.
    pub vertices: Vec<Vertex>,
    #[serde(rename = "f")]
    /// The faces, represented as lists of indexes into `vertices` above.
    pub faces: Vec<Vec<u32>>,
}
