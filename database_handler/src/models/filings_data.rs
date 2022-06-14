use diesel::pg::expression::array_comparison::All;
use diesel::prelude::*;
use diesel::pg::types::sql_types::Jsonb;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::database;
use crate::database_errors::DbError;
use crate::schema::filings_data;
use crate::models::all_filings::AllFilings;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DHoldings {

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DTransactions {

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NDHoldings {

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NDTransactions {

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Footnotes {

}



#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[table_name = "filings_data"]
#[primary_key(accession_number)]
#[belongs_to(AllFilings, foreign_key = "accession_number")]
pub struct FilingsData {
    pub accession_number: i64,
    pub d_holdings: DHoldings,
    pub d_transactions: DTransactions,
    pub nd_holdings: NDHoldings,
    pub nd_transactions: NDTransactions,
    pub footnotes: Footnotes,
}

impl FilingsData {  }