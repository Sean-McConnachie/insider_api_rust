use diesel::pg::expression::array_comparison::All;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::{all_filings, json_docs, stock_data};
use crate::models::stock_data::StockData;
use crate::models::json_docs::JsonDocs;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Insertable, Debug)]
#[table_name = "all_filings"]
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

impl AllFilings {
    pub fn insert_many(data: Vec<Self>) -> Result<(), DbError> {
        let conn = database::connection()?;

        conn.transaction::<_, diesel::result::Error, _>(|| {
            // First insert the items
            diesel::insert_into(all_filings::table)
                .values(&data)
                .execute(&conn)?;
            // Then set the fulfilled colum in json_docs to true
            println!("UPDATE HERE");
            /*
            let x = JsonDocs::belonging_to(&data)
                .update(jsondocs::fulfilled.eq(true))
                .execute(&conn)?;
            */
            Err(diesel::result::Error::RollbackTransaction)
            //Ok(())
        })?;
        Ok(())
    }

    pub fn insert(data: Self) -> Result<(), DbError> {
        Self::insert_many(vec![data])
    }
}