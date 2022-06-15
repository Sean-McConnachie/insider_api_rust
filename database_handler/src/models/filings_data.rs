use diesel::pg::expression::array_comparison::All;
use diesel::prelude::*;
use diesel::pg::types::sql_types::Jsonb;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::database;
use crate::database_errors::DbError;
use crate::schema::{filings_data, all_filings, insider_roles, all_insiders};
use crate::models::all_filings::AllFilings;
use crate::models::insider_roles::InsiderRoles;
use crate::models::all_insiders::AllInsiders;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DHolding {
    pub security_title: String,
    pub price: f32,
    pub exercise_date: Option<i64>,
    pub expiration_date: Option<i64>,
    pub underlying_security_title: String,
    pub underlying_security_price: f32,
    pub ownership_nature: String,
    pub indirect_relation: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NdHolding {
    pub security_title: String,
    pub post_transaction_shares: f32,
    pub ownership_nature: String,
    pub indirect_relation: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DTransaction {
    pub security_title: String,
    pub price: f32,
    pub transaction_date: Option<i64>,
    pub deemed_execution_date: Option<i64>,
    pub transaction_code: String,
    pub transaction_equity_swap: bool,
    pub transaction_shares: f32,
    pub transaction_share_price: f32,
    pub transaction_ad_code: String,
    pub underlying_security_title: String,
    pub underlying_security_price: f32,
    pub post_transaction_shares: f32,
    pub exercise_date: Option<i64>,
    pub expiration_date: Option<i64>,
    pub ownership_nature: String,
    pub indirect_relation: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NdTransaction {
    pub security_title: String,
    pub transaction_date: Option<i64>,
    pub transaction_code: String,
    pub transaction_equity_swap: bool,
    pub transaction_shares: f32,
    pub transaction_share_price: f32,
    pub transaction_ad_code: String,
    pub post_transaction_shares: f32,
    pub ownership_nature: String,
    pub indirect_relation: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Footnote {
    pub footnote_id: String,
    pub text: String
}

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug, Insertable)]
#[table_name = "filings_data"]
#[primary_key(accession_number)]
#[belongs_to(AllFilings, foreign_key = "accession_number")]
pub struct FilingsData {
    pub accession_number: i64,
    pub d_holdings: Value,
    pub d_transactions: Value,
    pub nd_holdings: Value,
    pub nd_transactions: Value,
    pub footnotes: Value,
}

impl FilingsData {
    pub fn insert(filings_documents: Self,
                  insider_documents: Vec<AllInsiders>,
                  roles_documents: Vec<InsiderRoles>,
                  accession_number: i64) -> Result<(), DbError> {
        let conn = database::connection()?;

        conn.transaction::<_, diesel::result::Error, _>(|| {

            diesel::insert_into(filings_data::table)
                .values(&filings_documents)
                .execute(&conn)?;

            diesel::insert_into(all_insiders::table)
                .values(&insider_documents)
                .execute(&conn)?;

            diesel::insert_into(insider_roles::table)
                .values(&roles_documents)
                .execute(&conn)?;

            // Then set the fulfilled colum in all_filings to true
            diesel::update(all_filings::table)
                .filter(all_filings::accession_number.eq(accession_number))
                .set(all_filings::fulfilled.eq(true))
                .execute(&conn)?;

            Ok(())
        })?;

        Ok(())
    }
}

