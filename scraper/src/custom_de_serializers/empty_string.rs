use serde::{self, Deserialize, Deserializer};
use serde::de::Error;


pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
{
    let o: Option<String> = Option::deserialize(deserializer)?;
    Ok(o.filter(|s| !s.is_empty()))
}