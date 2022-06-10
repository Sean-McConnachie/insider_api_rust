use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::dholdings;
use crate::models::all_filings::AllFilings;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "dholdings"]
#[belongs_to(AllFilings, foreign_key = "accession_number")]
pub struct DHoldings {
    pub id: i32,
    pub accession_number: i64,
    pub security_title: String,
    pub price: f32,
    pub exercise_date: Option<i64>,
    pub expiration_date: Option<i64>,
    pub underlying_security_title: String,
    pub underlying_security_price: f32,
    pub ownership_nature: String,
    pub indirect_relation: Option<String>
}

impl DHoldings { }