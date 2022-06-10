use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::jsondocs;
use crate::models::stock_data::StockData;

#[derive(Identifiable, Associations, Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "jsondocs"]
#[belongs_to(StockData, foreign_key = "company_cik")]
pub struct JsonDocs {
    pub id: i32,
    pub company_cik: i32,
    pub url: String,
    pub old: bool,
    pub fulfilled: bool
}

impl JsonDocs { }