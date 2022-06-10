use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::ndtransactions;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "ndtransactions"]
#[belongs_to(AllFilings, foreign_key = "accession_number")]
pub struct NDTransactions {
    pub id: i32,
    pub accession_number: i64,
    pub security_title: String,
    pub transaction_date: Option<i64>,
    pub transaction_code: String,
    pub transaction_equity_swap: bool,
    pub transaction_shares: f32,
    pub transaction_share_price: f32,
    pub transaction_ad_code: String,
    pub post_transaction_total_shares: f32,
    pub ownership_nature: String,
    pub indirect_relation: Option<String>
}

impl NDTransactions { }