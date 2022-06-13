use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::ndholdings;
use crate::models::all_filings::AllFilings;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "ndholdings"]
#[belongs_to(AllFilings, foreign_key = "accession_number")]
pub struct NdHoldings {
    pub id: Option<i32>,
    pub accession_number: i64,
    pub security_title: String,
    pub post_transaction_amount: f32,
    pub ownership_nature: String,
    pub indirect_relation: Option<String>
}

impl NdHoldings { }