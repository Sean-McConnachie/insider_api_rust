use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::dtransactions;
use crate::models::all_filings::AllFilings;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "dtransactions"]
#[belongs_to(AllFilings, foreign_key = "accession_number")]
pub struct DTransactions {
    pub id: Option<i32>,
    pub accession_number: i64,
    pub security_title: String,
    pub price: f32,
    pub transaction_date: Option<i64>,
    pub deemed_execution_date: Option<i64>,
    pub transaction_code: String,
    pub transaction_equity_swap: bool,
    pub transaction_shares: f32,
    pub transaction_share_price: f32,
    pub transaction_ad_code: String,
    pub exercise_date: Option<i64>,
    pub expiration_date: Option<i64>,
    pub underlying_security_title: String,
    pub underlying_security_price: f32,
    pub post_transaction_total_shares: f32,
    pub ownership_nature: String,
    pub indirect_relation: Option<String>
}

impl DTransactions { }