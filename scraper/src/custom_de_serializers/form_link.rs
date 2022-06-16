use serde::{self, Deserialize, Deserializer, Serializer};
use serde::de::Error;


pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
{
    let values: Option<Vec<String>> = Option::deserialize(deserializer)?;
    let mut parsed = Vec::<String>::new();
    return if values == None {
        Err(Error::custom("Failed to convert datetime to i64"))
    } else {
        for val in values.unwrap() {
            parsed.push(if val.starts_with("xslF") {
                let x = val[11..].to_string();
                x
            } else {
                val
            });
        }
        Ok(parsed)
    }
}
