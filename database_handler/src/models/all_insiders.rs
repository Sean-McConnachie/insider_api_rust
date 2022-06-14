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

impl AllInsiders { }