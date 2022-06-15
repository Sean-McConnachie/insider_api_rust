use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::insider_roles;
use crate::models::stock_data::StockData;
use crate::models::all_insiders::AllInsiders;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "insider_roles"]
#[belongs_to(AllInsiders, foreign_key = "insider_cik")]
#[belongs_to(StockData, foreign_key = "company_cik")]
pub struct InsiderRoles {
    pub id: Option<i32>,
    pub insider_cik: i32,
    pub company_cik: i32,
    pub director: bool,
    pub officer: bool,
    pub ten_percent: bool,
    pub other: bool,
    pub officer_title: Option<String>,
    pub other_text: Option<String>,
    pub str1: String,
    pub str2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub state_description: Option<String>
}

impl InsiderRoles {
    pub fn exists(insider_cik: i32, company_cik: i32) -> Result<bool, DbError> {
        let conn = database::connection()?;

        let exists = insider_roles::table
            .filter(insider_roles::insider_cik.eq(&insider_cik))
            .filter(insider_roles::company_cik.eq(&company_cik))
            .select(insider_roles::insider_cik)
            .get_results::<i32>(&conn)?;

        match exists.len() {
            0 => Ok(false),
            _ => Ok(true)
        }
    }
}