use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::all_insiders;

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
}