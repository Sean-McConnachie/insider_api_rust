use diesel::dsl::any;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::all_insiders;

use crate::parsing::csv::csv_to_type;


#[derive(Identifiable, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "all_insiders"]
#[primary_key(insider_cik)]
pub struct AllInsiders {
    pub insider_cik: i32,
    pub name: String
}

impl AllInsiders {
    pub fn exists(insider_cik: i32) -> Result<bool, DbError> {
        let conn = database::connection()?;

        let exists = all_insiders::table
            .filter(all_insiders::insider_cik.eq(&insider_cik))
            .select(all_insiders::insider_cik)
            .get_results::<i32>(&conn)?;

        match exists.len() {
            0 => Ok(false),
            _ => Ok(true)
        }
    }

    pub fn select_parameters(params: AllInsidersParams) -> Result<Vec<Self>, DbError> {
        let insider_ciks: Vec<i32> = csv_to_type(params.insider_ciks, "insider_ciks")?;

        let conn = database::connection()?;

        let mut query = all_insiders::table.into_boxed();
        if !insider_ciks.is_empty() {
            query = query.filter(all_insiders::insider_cik.eq(any(insider_ciks)));
        }

        let r = query.load(&conn)?;
        Ok(r)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AllInsidersParams {
    #[serde(alias = "insider_cik")]
    pub insider_ciks: Option<String>
}