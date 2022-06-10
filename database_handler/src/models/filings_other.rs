use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::filingsother;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "filingsother"]
#[belongs_to(AllFilings, foreign_key = "accession_number")]
pub struct NDTransactions {
    pub id: i32,
    pub accession_number: i64,
    pub footnotes_json: String
}

impl NDTransactions { }