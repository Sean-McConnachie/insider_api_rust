use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database_errors::DbError;
use crate::schema::filingsother;
use crate::models::all_filings::AllFilings;

#[derive(Identifiable, Associations, Serialize, Deserialize, Queryable, Insertable, Debug)]
#[table_name = "filingsother"]
#[primary_key(accession_number)]
#[belongs_to(AllFilings, foreign_key = "accession_number")]
pub struct FilingsOther {
    pub accession_number: i64,
    pub footnotes_json: String
}

impl FilingsOther { }