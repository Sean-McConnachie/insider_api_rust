use std::fmt::Debug;
use quick_xml;
use anyhow;

use database_handler::models::{stock_data, all_filings};
use request_handler::{QueueRequest, RequestData};

use crate::{Insider};
use crate::CallbackError;
use crate::models::rss_response;


#[derive(Clone, Debug)]
struct RssReqData {
    pub cik: i32,
}

impl Insider {
    pub async fn run_rss(&mut self) -> Result<(), CallbackError> {
        // Find unfulfilled requests in the db where old = false + make requests + insert to database
        self.rss_make_request().await?;
        Ok(())
    }

    async fn rss_make_request(&self) -> Result<(), CallbackError> {
        let ciks = stock_data::StockData::select_ciks()?;

        let requests = self.rss_create_requests(ciks);

        self.queue_request(requests,
                           self.config.sec.delay_milli,
                           self.config.sec.concurrent,
                           Insider::rss_callback).await?;

        Ok(())
    }

    fn rss_create_requests(&self, company_ciks: Vec<i32>) -> Vec<RequestData<RssReqData>> {
        let mut requests = Vec::new();
        for (i, cik) in company_ciks.iter().enumerate() {
            let mut url = self.config.sec.rss_feed_url
                .replacen("{}", &cik.to_string(), 1);
            url = url
                .replacen("{}", &self.config.sec.rss_count.to_string(), 1);
            let request = RequestData {
                url,
                headers: self.config.sec.headers.clone(),
                count: i,
                data: RssReqData { cik: *cik }
            };
            requests.push(request);
        }
        requests
    }

    fn rss_callback(response_slice: Vec<u8>, request_data: RequestData<RssReqData>)
                            -> Result<(), anyhow::Error>
    {
        let rss_data: rss_response::Response = quick_xml::de::from_slice(&response_slice).unwrap();
        let company_cik = request_data.data.cik;

        let existing_accession_numbers = all_filings::AllFilings::select_accession_numbers_by_cik(company_cik)?;

        let mut document_inserts = Vec::new();
        for entry in rss_data.entries {
            if existing_accession_numbers.contains(&entry.content.accession_number) { continue; }

            document_inserts.push(all_filings::AllFilings {
                accession_number: entry.content.accession_number,
                acceptance_datetime: entry.content.acceptance_date_time,
                filing_date: entry.content.filing_date,
                report_date: entry.content.report_date,
                size: entry.content.size,
                company_cik,
                form_link: None,
                index_link: Some(entry.content.index_link),
                form_type: entry.category.form_type,
                fulfilled: false,
                insider_ciks: vec![]
            });
        }

        all_filings::AllFilings::insert_update_transaction(document_inserts, company_cik, false).expect("Failed to insert documents");

        Ok(())
    }
}