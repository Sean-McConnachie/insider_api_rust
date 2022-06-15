use serde::{self, Deserialize, Deserializer};
use serde::de::Error;


pub fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
    where
        D: Deserializer<'de>,
{

    let values: Option<Vec<String>> = Option::deserialize(deserializer)?;
    let mut parsed = Vec::<i64>::new();
    return if values == None {
        Err(Error::custom("Failed to convert accession number to i64"))
    } else {
        for val in values.unwrap() {
            let val = val.replace("-", "");
            match val.parse::<i64>() {
                Ok(v) => parsed.push(v),
                _ => return Err(Error::custom("Failed to convert accession number to i64"))
            };
        }
        Ok(parsed)
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
{

    let val: Option<String> = Option::deserialize(deserializer)?;
    return if val == None {
        Err(Error::custom("Failed to convert accession number to i64"))
    } else {
        let val = val.unwrap().replace("-", "");
        match val.parse::<i64>() {
            Ok(v) => Ok(v),
            _ => Err(Error::custom("Failed to convert accession number to i64"))
        }
    }
}