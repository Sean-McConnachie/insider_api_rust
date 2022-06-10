use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::stockdata;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "stockdata"]
#[primary_key(company_cik)]
pub struct StockData {
    pub company_cik: i32,
    pub ticker: String,
    pub exchange: String,
    pub short_name: String,
    pub full_name: String,
    pub isin: String
}

impl StockData {
    pub fn insert_many(data: Vec<StockData>) -> Result<usize, DbError> {
        let conn = database::connection()?;
        let u = diesel::insert_into(stockdata::table)
            .values(&data)
            .execute(&conn)?;
        Ok(u)
    }

    pub fn insert(data: StockData) -> Result<usize, DbError> {
        StockData::insert_many(vec![data])
    }
}