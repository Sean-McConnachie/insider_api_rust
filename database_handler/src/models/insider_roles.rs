use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::insiderroles;
use crate::models::stock_data::StockData;
use crate::models::all_insiders::AllInsiders;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "insiderroles"]
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

impl InsiderRoles { }