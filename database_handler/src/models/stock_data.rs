use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::stockdata;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Insertable)]
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
    /*
    pub fn find_all() -> Result<Vec<Self>, DbError> {
        let conn = database::connection()?;

        let users = user::table
            .load::<User>(&conn)?;

        Ok(users)
    }
     */
}