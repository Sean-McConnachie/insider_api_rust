use serde::{Serialize, Deserialize};

use crate::custom_de_serializers::accession_number;
use crate::custom_de_serializers::date;
use crate::custom_de_serializers::datetime;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    #[serde( rename = "entry", default)]
    pub entries: Vec<Entry>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry {
    pub category: Category,
    #[serde( rename = "content-type")]
    pub content: Content,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Category {
    #[serde( rename = "term" )]
    pub form_type: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    #[serde(deserialize_with  = "datetime::deserialize")]
    #[serde( rename = "acceptance-date-time" )]
    pub acceptance_date_time: i64,

    #[serde(deserialize_with  = "accession_number::deserialize")]
    #[serde( rename = "accession-number" )]
    pub accession_number: i64,

    #[serde(deserialize_with  = "date::deserialize")]
    #[serde( rename = "filing-date" )]
    pub filing_date: i64,

    #[serde( rename = "filing-href" )]
    pub index_link: String,

    #[serde(deserialize_with  = "date::deserialize")]
    #[serde( rename = "report-date" )]
    pub report_date: i64,

    #[serde( rename = "size" )]
    pub size: i32
}
