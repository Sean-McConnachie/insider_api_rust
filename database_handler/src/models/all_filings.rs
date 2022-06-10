use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::allfilings;
use crate::models::stock_data::StockData;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "allfilings"]
#[primary_key(accession_number)]
#[belongs_to(StockData, foreign_key = "company_cik")]
pub struct AllFilings {
    pub accession_number: i64,
    pub acceptance_datetime: i64,
    pub filing_date: i64,
    pub report_date: i64,
    pub size: i32,
    pub company_cik: i32,
    pub form_link: Option<String>,
    pub index_link: Option<String>,
    pub form_type: String,
    pub fulfilled: bool,
}

impl AllFilings { }