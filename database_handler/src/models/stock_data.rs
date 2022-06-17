use diesel::{debug_query, sql_query};
use diesel::dsl::any;
use diesel::prelude::*;
use diesel::types::Json;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::models::json_docs::JsonDocs;
use crate::schema::{json_docs, stock_data};

use crate::parsing::csv::{csv_to_type};


#[derive(Identifiable, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "stock_data"]
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
    pub fn insert_many(data: Vec<Self>) -> Result<usize, DbError> {
        let conn = database::connection()?;
        let u = diesel::insert_into(stock_data::table)
            .values(&data)
            .execute(&conn)?;
        Ok(u)
    }

    pub fn insert(data: Self) -> Result<usize, DbError> {
        Self::insert_many(vec![data])
    }

    pub fn select_ciks() -> Result<Vec<i32>, DbError> {
        let conn = database::connection()?;
        let u = stock_data::table
            .select(stock_data::company_cik)
            .load(&conn)?;
        Ok(u)
    }

    pub fn select_parameters(params: CompaniesParams) -> Result<Vec<Self>, DbError> {
        let company_ciks: Vec<i32> = csv_to_type(params.company_ciks, "company_ciks")?;
        let tickers: Vec<String> = csv_to_type(params.tickers, "tickers")?;
        let exchanges: Vec<String> = csv_to_type(params.exchanges, "exchanges")?;
        let isins: Vec<String> = csv_to_type(params.isins, "isins")?;

        let conn = database::connection()?;

        let mut query = stock_data::table.into_boxed();
        if !company_ciks.is_empty() {
            query = query.filter(stock_data::company_cik.eq(any(company_ciks)));
        }
        if !tickers.is_empty() {
            query = query.filter(stock_data::ticker.eq(any(tickers)));
        }
        if !exchanges.is_empty() {
            query = query.filter(stock_data::exchange.eq(any(exchanges)));
        }
        if !isins.is_empty() {
            query = query.filter(stock_data::isin.eq(any(isins)));
        }

        let r = query.load(&conn)?;
        Ok(r)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompaniesParams {
    #[serde(alias = "company_cik")]
    pub company_ciks: Option<String>,

    #[serde(alias = "ticker")]
    pub tickers: Option<String>,

    #[serde(alias = "exchange")]
    pub exchanges: Option<String>,

    #[serde(alias = "isin")]
    pub isins: Option<String>
}