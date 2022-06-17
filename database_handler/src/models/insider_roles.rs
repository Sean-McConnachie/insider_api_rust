use std::any::Any;
use diesel::dsl::any;
use diesel::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::database;
use crate::database_errors::DbError;
use crate::schema::insider_roles;
use crate::models::stock_data::StockData;
use crate::models::all_insiders::AllInsiders;

use crate::parsing::csv::csv_to_type;


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

    pub fn select_parameters(params: InsidersRolesParams) -> Result<Value, DbError>
    {
        let insider_ciks: Vec<i32> = csv_to_type(params.insider_ciks, "insider_ciks")?;
        let company_ciks: Vec<i32> = csv_to_type(params.company_ciks, "company_ciks")?;

        let conn = database::connection()?;

        let mut query = insider_roles::table
            .select((
                insider_roles::insider_cik,
                insider_roles::company_cik,
                insider_roles::director,
                insider_roles::officer,
                insider_roles::ten_percent,
                insider_roles::other,
                insider_roles::officer_title,
                insider_roles::other_text,
                insider_roles::str1,
                insider_roles::str2,
                insider_roles::city,
                insider_roles::state,
                insider_roles::zip,
                insider_roles::state_description))
                .into_boxed();

        if !insider_ciks.is_empty() {
            query = query.filter(insider_roles::insider_cik.eq(any(insider_ciks)));
        }
        if !company_ciks.is_empty() {
            query = query.filter(insider_roles::company_cik.eq(any(company_ciks)));
        }

        if params.director != None {
            query = query.filter(insider_roles::director.eq(params.director.unwrap()));
        }
        if params.officer != None {
            query = query.filter(insider_roles::officer.eq(params.officer.unwrap()));
        }
        if params.ten_percent != None {
            query = query.filter(insider_roles::ten_percent.eq(params.ten_percent.unwrap()));
        }
        if params.other != None {
            query = query.filter(insider_roles::other.eq(params.other.unwrap()));
        }

        let r = query.load::<InsiderRolesNoId>(&conn)?;
        let r = serde_json::to_value(&r)?;

        Ok(r)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InsidersRolesParams {
    #[serde(alias = "insider_cik")]
    pub insider_ciks: Option<String>,

    #[serde(alias = "company_cik")]
    pub company_ciks: Option<String>,

    pub director: Option<bool>,
    pub officer: Option<bool>,
    pub ten_percent: Option<bool>,
    pub other: Option<bool>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct InsiderRolesNoId {
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