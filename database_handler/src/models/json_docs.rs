use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::dsl::{not, sql};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::jsondocs;
use crate::models::stock_data::StockData;


#[derive(Identifiable, Associations, Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name = "jsondocs"]
#[belongs_to(StockData, foreign_key = "company_cik")]
pub struct JsonDocs {
    pub id: Option<i32>,
    pub company_cik: i32,
    pub url: String,
    pub old: bool,
    pub fulfilled: bool
}

impl JsonDocs {
    pub fn insert_many(data: Vec<Self>) -> Result<usize, DbError> {
        let conn = database::connection()?;

        let u = diesel::insert_into(jsondocs::table)
            .values(&data)
            .execute(&conn)?;
        Ok(u)
    }

    pub fn insert(data: Self) -> Result<usize, DbError> {
        Self::insert_many(vec![data])
    }

    pub fn select_not_in_jsondocs() -> Result<Vec<i32>, DbError> {
        let conn = database::connection()?;

        let q = sql(r#"
            SELECT stockdata.company_cik
            FROM stockdata
            WHERE stockdata.company_cik NOT IN (SELECT jsondocs.company_cik FROM jsondocs);"#)
            .get_results::<i32>(&conn)?;

        Ok(q)
    }

    pub fn old_select(old: bool) -> Result<Vec<Self>, DbError> {
        let conn = database::connection()?;

        let resp = jsondocs::table
            .filter(jsondocs::fulfilled.eq(&false))
            .filter(jsondocs::old.eq(&old))
            .load::<Self>(&conn)?;

        Ok(resp)
    }
}