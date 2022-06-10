use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::jsondocs;

#[derive(Identifiable, Associations, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "jsondocs"]
#[belongs_to(StockData, foreign_key = "company_cik")]
pub struct JsonDocs {
    pub id: i32,
    pub company_cik: String,
    pub url: String,
    pub old: bool,
    pub fulfilled: bool
}

impl JsonDocs { }