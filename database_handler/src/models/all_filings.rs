use diesel::dsl::any;
use diesel::pg::expression::array_comparison::All;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::database;
use crate::database_errors::DbError;
use crate::models::filings_data::FilingsData;
use crate::schema::{all_filings, json_docs, stock_data, filings_data};
use crate::models::stock_data::StockData;
use crate::models::json_docs::JsonDocs;
use crate::parsing::csv::{convert_to_type, csv_to_type, date_option};

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Insertable, Debug)]
#[table_name = "all_filings"]
#[primary_key(accession_number)]
#[belongs_to(StockData, foreign_key = "company_cik")]
pub struct AllFilings {
    pub accession_number: i64,
    pub acceptance_datetime: i64,
    pub filing_date: i64,
    pub report_date: i64,
    pub size: i32,
    pub company_cik: i32,
    pub form_link: Option<String>,
    pub index_link: Option<String>,
    pub form_type: String,
    pub fulfilled: bool,
    pub insider_ciks: Vec<i32>,
}


#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct AccessionIndex {
    pub index_link: Option<String>,
    pub accession_number: i64
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct AccessionForm {
    pub form_link: Option<String>,
    pub accession_number: i64,
    pub company_cik: i32
}

impl AllFilings {
    pub fn insert_update_transaction(data: Vec<Self>, company_cik: i32, old: bool) -> Result<(), DbError>
    {
        let conn = database::connection()?;

        conn.transaction::<_, diesel::result::Error, _>(|| {
            // First insert the items
            diesel::insert_into(all_filings::table)
                .values(&data)
                .execute(&conn)?;
            // Then set the fulfilled colum in json_docs to true
            diesel::update(json_docs::table)
                .filter(json_docs::company_cik.eq(company_cik))
                .filter(json_docs::old.eq(old))
                .set(json_docs::fulfilled.eq(true))
                .execute(&conn)?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn update_form_link(accession_number: i64, form_link: String) -> Result<(), DbError>
    {
        let conn = database::connection()?;

        diesel::update(all_filings::table)
            .filter(all_filings::accession_number.eq(accession_number))
            .set(all_filings::form_link.eq(form_link))
            .execute(&conn)?;

        Ok(())
    }

    pub fn select_accession_numbers_by_cik(company_cik: i32) -> Result<Vec<i64>, DbError>
    {
        let conn = database::connection()?;
        let accession_numbers = all_filings::table
            .select(all_filings::accession_number)
            .filter(all_filings::company_cik.eq(company_cik))
            .load::<i64>(&conn)?;
        Ok(accession_numbers)
    }

    pub fn select_accession_index() -> Result<Vec<AccessionIndex>, DbError>
    {
        let conn = database::connection()?;
        let u = all_filings::table
            .select((all_filings::index_link,
                     all_filings::accession_number))
            .filter(all_filings::form_link.is_null())
            .filter(all_filings::index_link.is_not_null())
            .filter(all_filings::fulfilled.eq(false))
            .load::<AccessionIndex>(&conn)?;
        Ok(u)
    }

    pub fn select_accession_form() -> Result<Vec<AccessionForm>, DbError>
    {
        let conn = database::connection()?;
        let u = all_filings::table
            .select((all_filings::form_link,
                     all_filings::accession_number,
                     all_filings::company_cik))
            .filter(all_filings::form_link.is_not_null())
            .filter(all_filings::fulfilled.eq(false))
            .load::<AccessionForm>(&conn)?;
        Ok(u)
    }

    pub fn select_parameters_no_tables(params: ParsedFilingParams) -> Result<Value, DbError>
    {
        let conn = database::connection()?;

        let mut query = all_filings::table.into_boxed();

        if !params.accession_numbers.is_empty() {
            query = query.filter(all_filings::accession_number.eq(any(params.accession_numbers)));
        }
        if !params.insider_ciks.is_empty() {
            query = query.filter(all_filings::insider_ciks.overlaps_with(params.insider_ciks))

        }
        if !params.company_ciks.is_empty() {
            query = query.filter(all_filings::company_cik.eq(any(params.company_ciks)))
        }
        if !params.form_types.is_empty() {
            query = query.filter(all_filings::form_type.eq(any(params.form_types)))
        }

        // TODO: What time to sort by? Filing data, report date or acceptance datetime?
        if params.time_range != None {
            let time_range = params.time_range.unwrap();
            if time_range.start_date != None {
                query = query.filter(all_filings::acceptance_datetime
                    .ge(time_range.start_date.unwrap()));
            }
            if time_range.end_date != None {
                query = query.filter(all_filings::acceptance_datetime
                    .le(time_range.end_date.unwrap()));
            }
        }


        let r = query.load::<Self>(&conn)?;
        Ok(serde_json::to_value(&r)?)
    }

    pub fn select_parameters_with_tables(params: ParsedFilingParams) -> Result<Value, DbError>
    {
        let conn = database::connection()?;

        let mut query = all_filings::table
            .inner_join(filings_data::table)
            .into_boxed();

        if !params.accession_numbers.is_empty() {
            query = query.filter(all_filings::accession_number.eq(any(params.accession_numbers)));
        }
        if !params.insider_ciks.is_empty() {
            query = query.filter(all_filings::insider_ciks.overlaps_with(params.insider_ciks))

        }
        if !params.company_ciks.is_empty() {
            query = query.filter(all_filings::company_cik.eq(any(params.company_ciks)))
        }
        if !params.form_types.is_empty() {
            query = query.filter(all_filings::form_type.eq(any(params.form_types)))
        }

        // TODO: What time to sort by? Filing data, report date or acceptance datetime?
        if params.time_range != None {
            let time_range = params.time_range.unwrap();
            if time_range.start_date != None {
                query = query.filter(all_filings::acceptance_datetime
                    .ge(time_range.start_date.unwrap()));
            }
            if time_range.end_date != None {
                query = query.filter(all_filings::acceptance_datetime
                    .le(time_range.end_date.unwrap()));
            }
        }


        let r = query.load::<AllFilingsData>(&conn)?;
        Ok(serde_json::to_value(&r)?)
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct AllFilingsData {
    #[serde(flatten)]
    pub all_filings: AllFilings,
    #[serde(flatten)]
    pub filings_data: FilingsData
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilingParams {
    #[serde(alias = "accession_number")]
    pub accession_numbers: Option<String>,

    #[serde(alias = "insider_cik")]
    pub insider_ciks: Option<String>,

    #[serde(alias = "company_cik")]
    pub company_ciks: Option<String>,

    #[serde(alias = "form_type")]
    pub form_types: Option<String>,

    pub start_date: Option<String>,
    pub end_date: Option<String>,

    pub period_range: Option<String>,
    pub period_time: Option<String>,
    pub period_go_back: Option<bool>
}

#[derive(Debug, Clone)]
pub struct ParsedFilingParams {
    pub accession_numbers: Vec<i64>,
    pub insider_ciks: Vec<i32>,
    pub company_ciks: Vec<i32>,
    pub form_types: Vec<String>,
    pub time_range: Option<TimeRange>,
}

impl TryFrom<FilingParams> for ParsedFilingParams {
    type Error = DbError;

    fn try_from(params: FilingParams) -> Result<Self, Self::Error> {
        let accession_numbers: Vec<i64> = csv_to_type(params.accession_numbers, "insider_ciks")?;
        let insider_ciks: Vec<i32> = csv_to_type(params.insider_ciks, "insider_ciks")?;
        let company_ciks: Vec<i32> = csv_to_type(params.company_ciks, "insider_ciks")?;
        let form_types: Vec<String> = csv_to_type(params.form_types, "insider_ciks")?;
        for form in &form_types {
            if form != "3" || form != "4" || form != "5" {
                return Err(DbError::ParseError(format!("Invalid form type: {}. Must be either `3` | `4` | `5`", form)));
            }
        }

        let period_range_possible = params.period_range != None && params.period_time != None;

        let time_range_specified = params.start_date != None || params.end_date != None;
        let period_range_specifed = params.period_range != None || params.period_time != None || params.period_go_back != None;

        let time_range = if time_range_specified && period_range_specifed {
            return Err(DbError::ParseError("Cannot specify both time range and period range".to_string()));
        } else if time_range_specified {
            let start_date = params.start_date.unwrap_or_default();
            let end_date = params.end_date.unwrap_or_default();

            if start_date == "" && end_date == "" {
                return Err(DbError::ParseError("Parameters cannot both be empty. If you would like to query all, please set extreme values or remove all time-related parameters.".to_string()));
            }

            let start_date = date_option(start_date, "start_date")?;
            let end_date = date_option(end_date, "end_date")?;

            if start_date != None && end_date != None {
                if start_date > end_date {
                    return Err(DbError::ParseError("Start date must be before end date".to_string()));
                }
            }

            Some(TimeRange { start_date, end_date })
        } else if period_range_possible {
            let mut period_range = params.period_range.unwrap();
            let period_time = params.period_time.unwrap();
            let period_go_back = params.period_go_back;

            if period_range == "" || period_time == "" {
                return Err(DbError::ParseError("Parameters must be filled. `period_range` and `period_time`".to_string()));
            }
            let range_char = period_range.chars().last().unwrap();
            let range_char = range_char.to_string().to_lowercase();
            let period_range = if ["d", "w", "m", "y"].contains(&&*range_char) {
                let range = period_range.trim_end_matches(&range_char);
                if range == "" { return Err(DbError::ParseError("Period range value not specified.".to_string())); }

                let x: i64 = convert_to_type(range.to_string(), "period_range")?;
                match range_char.as_str() { // TODO: MILISECONDS!!!!
                    "d" => x * 24 * 60 * 60 * 1000,
                    "w" => x * 7 * 24 * 60 * 60 * 1000,
                    "m" => x * 30 * 24 * 60 * 60 * 1000,
                    "y" => x * 365 * 24 * 60 * 60 * 1000,
                    _ => unreachable!()
                }
            } else {
                return Err(DbError::ParseError("Invalid period range. Must be one of `d`, `w`, `m`, `y`".to_string()));
            };
            let period_time: i64 = convert_to_type(period_time, "period_time")?;

            let period_go_back = match period_go_back {
                Some(v) => v,
                None => true, // Default to go back
            };

            // We have valid period_range and period_time and period_go_back
            let (start_date, end_date) = if period_go_back == true {
                (period_time - period_range, period_time)
            } else {
                (period_time, period_time + period_range)
            };
            Some(TimeRange { start_date: Some(start_date), end_date: Some(end_date) })

        } else {
            // Can be any time
            None
        };

        Ok(Self {
            accession_numbers,
            insider_ciks,
            company_ciks,
            form_types,
            time_range,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeRange {
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}