use chrono::{TimeZone, Utc};
use serde::{self, Deserialize, Deserializer};
use serde::de::Error;


pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
    where
        D: Deserializer<'de>,
{

    let values: Option<Vec<String>> = Option::deserialize(deserializer)?;
    let mut parsed = Vec::<i64>::new();
    return if values == None {
        Err(Error::custom("Failed to convert date to i64"))
    } else {
        for val in values.unwrap() {
            if val == "" {
                parsed.push(0);
                continue
            }
            let val = format!("{} 00:00:00", val);
            match Utc.datetime_from_str(&val, "%Y-%m-%d %H:%M:%S") {
                Ok(v) => parsed.push(v.timestamp_millis()),
                _ => return Err(Error::custom("Failed to convert date to i64"))
            };
        }
        Ok(parsed)
    }
}
