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
    pub fn insert_update_transaction(data: Vec<Self>, company_cik: i32, old: bool) -> Result<(), DbError> {
        let conn = database::connection()?;

        conn.transaction::<_, diesel::result::Error, _>(|| {
            // First insert the items
            let x = diesel::insert_into(all_filings::table)
                .values(&data)
                .execute(&conn)?;
            // Then set the fulfilled colum in json_docs to true
            diesel::update(json_docs::table)
                .filter(json_docs::company_cik.eq(company_cik))
                .filter(json_docs::old.eq(old))
                .set(json_docs::fulfilled.eq(true))
                .execute(&conn)?;
            Ok(())
        })?;
        Ok(())
    }

}