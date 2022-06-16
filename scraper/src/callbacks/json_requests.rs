use std::fmt::Debug;
use std::fs;
use std::process::exit;
use std::time::Instant;

use async_trait::async_trait;
use hyper::{Client, HeaderMap};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use thiserror::Error;
use anyhow;

use database_handler::database_errors::DbError;
use database_handler::models::all_filings::AllFilings;
use database_handler::models::json_docs;
use request_handler::{QueueRequest, RequestData};

use crate::{Insider, InsiderError};
use crate::CallbackError;
use crate::models::json_response;
use crate::models::other_models;

#[derive(Clone, Debug)]
struct JsonReqData {
    pub cik: i32,
    pub xml_url: String,
    pub json_subsequent_url: String
}


impl Insider {
    pub async fn run_json(&mut self) -> Result<(), CallbackError> {
        // Finds the company_cik(s) that are found in stockdata but not in jsondocs
        self.json_generate_diff()?;

        // Find unfulfilled requests in the db where old = false + make requests + insert to database
        self.json_make_request(false).await?;
        self.json_make_request(true).await?;

        Ok(())
    }

    fn json_generate_diff(&self) -> Result<(), CallbackError> {
        let diff_ciks = json_docs::JsonDocs::select_not_in_jsondocs()?;
        let recent_jsons = diff_ciks.iter().map(|company_cik| {
            json_docs::JsonDocs {
                id: None,
                company_cik: *company_cik,
                url: self.config.sec.json_recent_url.replace("{}", format!("{:0>10}", company_cik).as_str()),
                old: false,
                fulfilled: false
            }
        }).collect::<Vec<_>>();
        if recent_jsons.len() > 0 { json_docs::JsonDocs::insert_many(recent_jsons.clone())?; }
        Ok(())
    }

    async fn json_make_request(&self, old: bool) -> Result<(), CallbackError> {
        let jsons = json_docs::JsonDocs::old_select(old)?;

        let requests = self.json_create_requests(jsons);

        let func = match old {
            false => Insider::json_recent_callback,
            true => Insider::json_subsequent_callback
        };

        self.queue_request(requests,
                           self.config.sec.delay_milli,
                           self.config.sec.concurrent,
                           func).await?;

        Ok(())
    }

    fn json_create_requests(&self, files: Vec<json_docs::JsonDocs>) -> Vec<RequestData<JsonReqData>> {
        let mut requests = Vec::new();
        for (i, file) in files.into_iter().enumerate() {
            requests.push(RequestData {
                data: JsonReqData {
                    cik: file.company_cik,
                    xml_url: self.config.sec.xml_url.clone(),
                    json_subsequent_url: self.config.sec.json_subsequent_url.clone()
                },
                url: file.url,
                headers: self.config.sec.headers.clone(),
                count: i,
            });
        }
        requests
    }

    fn json_recent_callback(response_slice: Vec<u8>, request_data: RequestData<JsonReqData>)
                            -> Result<(), anyhow::Error>
    {
        let json_data: json_response::FullResponse = serde_json::from_slice(&response_slice)?;

        let subsequent_jsons = json_data.filings.files.iter().map(|file| {
            json_docs::JsonDocs {
                id: None,
                company_cik: request_data.data.cik,
                url: request_data.data.json_subsequent_url.replace("{}", &*file.name),
                old: true,
                fulfilled: false
            }
        }).collect::<Vec<_>>();

        if subsequent_jsons.len() > 0 {
            json_docs::JsonDocs::insert_many(subsequent_jsons)?;
        }

        Insider::json_parse_insert(&request_data, &json_data.filings.recent, false)?;
        Ok(())
    }

    fn json_subsequent_callback(response_slice: Vec<u8>, request_data: RequestData<JsonReqData>)
                                -> Result<(), anyhow::Error>
    {
        let json_data: json_response::Data = serde_json::from_slice(&response_slice)?;
        Insider::json_parse_insert(&request_data, &json_data, true)?;
        Ok(())
    }

    fn json_parse_insert(request_data: &RequestData<JsonReqData>,
                         filing_data: &json_response::Data,
                         old: bool) -> Result<(), CallbackError>
    {
        let filing_data: &json_response::Data = filing_data;
        let company_cik = request_data.data.cik;
        let xml_url = request_data.data.xml_url.replacen("{}", &*format!("{:0>10}", &company_cik), 1);

        let mut document_inserts = Vec::<AllFilings>::new();
        for index in 0..filing_data.filing_date.len() {
            let form_type = filing_data.form_type[index].clone();
            if form_type == "3" || form_type == "4" || form_type == "5" {
                if filing_data.form_path[index].ends_with(".htm") || filing_data.form_path[index].ends_with(".html") {
                    continue;
                }
                let mut form_link = xml_url.replacen("{}", &*format!("{:0>18}", &filing_data.accession_number[index]), 1);
                form_link = form_link.replacen("{}", &filing_data.form_path[index], 1);
                document_inserts.push(AllFilings {
                    accession_number: filing_data.accession_number[index],
                    acceptance_datetime: filing_data.acceptance_date_time[index],
                    filing_date: filing_data.filing_date[index],
                    report_date: filing_data.report_date[index],
                    size: filing_data.size[index],
                    company_cik,
                    form_link: Some(form_link),
                    index_link: None,
                    form_type: form_type.to_string(),
                    fulfilled: false,
                });
            }
        }

        AllFilings::insert_update_transaction(document_inserts, company_cik, old)?;
        Ok(())
    }
}