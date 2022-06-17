use chrono::{TimeZone, Utc};
use serde::{self, Deserialize, Deserializer};
use serde::de::Error;


pub fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
    where
        D: Deserializer<'de>,
{
    let values: Option<Vec<String>> = Option::deserialize(deserializer)?;
    let mut parsed = Vec::<i64>::new();
    return if values == None {
        Err(Error::custom("Failed to convert datetime to i64"))
    } else {
        for val in values.unwrap() {
            if val == "" {
                parsed.push(0);
                continue
            }
            match Utc.datetime_from_str(&val, "%Y-%m-%dT%H:%M:%S.000Z") {
                Ok(v) => parsed.push(v.timestamp_millis()),
                _ => {
                    return Err(Error::custom("Failed to convert datetime to i64"))
                }
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
        Err(Error::custom("Failed to convert datetime to i64"))
    } else {
        let val = val.unwrap();
        if val == "" { return Ok(0); }
        match Utc.datetime_from_str(&val, "%Y-%m-%dT%H:%M:%S.000Z") {
            Ok(v) => Ok(v.timestamp_millis()),
            _ => Err(Error::custom("Failed to convert datetime to i64"))
        }
    }
}