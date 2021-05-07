//! Utilities for object [de]serialization.
//! Ripped shamelessly from coinbase-pro-rs
use serde::de::{self, Deserializer, Visitor};
use std::fmt;

struct F64InQuotes;

impl<'de> Visitor<'de> for F64InQuotes {
    type Value = f64;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("f64 as a number or string")
    }

    fn visit_f64<E>(self, id: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(id)
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        s.parse().map_err(de::Error::custom)
    }
}

pub fn f64_from_string<'de, D>(d: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(F64InQuotes)
}

#[allow(unused)]
pub fn f64_opt_from_string<'de, D>(d: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(F64InQuotes).map(Some).or(Ok(None))
}
