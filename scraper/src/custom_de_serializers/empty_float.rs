use serde::{self, Deserialize, Deserializer};


pub fn deserialize<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
{
    let o: Option<String> = Option::deserialize(deserializer)?;
    if o == None { return Ok(0.0); }
    else {
        let o = o.unwrap();
        if o == "" { return Ok(0.0); }
        else {
            let f: f32 = o.parse().unwrap();
            Ok(f)
        }
    }
}