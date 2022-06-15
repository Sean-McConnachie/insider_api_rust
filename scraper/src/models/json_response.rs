use chrono::NaiveDate;
use serde::{Deserialize};

use crate::custom_de_serializers::accession_number;
use crate::custom_de_serializers::date;
use crate::custom_de_serializers::datetime;
use crate::custom_de_serializers::form_link;


#[derive(Deserialize, Clone, Debug)]
pub struct FullResponse {
    pub filings: FilingTypes
}

#[derive(Deserialize, Clone, Debug)]
pub struct FilingTypes {
    pub recent: Data,
    pub files: Vec<Files>
}

#[derive(Deserialize, Clone, Debug)]
pub struct Files {
    pub name: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct Data {
    #[serde(deserialize_with  = "accession_number::deserialize_vec")]
    #[serde(rename = "accessionNumber")]
    pub accession_number: Vec<i64>,

    #[serde(deserialize_with  = "date::deserialize_vec")]
    #[serde(rename = "filingDate")]
    pub filing_date: Vec<i64>,

    #[serde(deserialize_with  = "date::deserialize_vec")]
    #[serde(rename = "reportDate")]
    pub report_date: Vec<i64>,

    #[serde(deserialize_with  = "datetime::deserialize_vec")]
    #[serde(rename = "acceptanceDateTime")]
    pub acceptance_date_time: Vec<i64>,

    #[serde(rename = "form")]
    pub form_type: Vec<String>,

    pub size: Vec<i32>,

    #[serde(deserialize_with  = "form_link::deserialize")]
    #[serde(rename = "primaryDocument")]
    pub form_path: Vec<String>
}