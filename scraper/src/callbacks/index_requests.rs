use std::fmt::Debug;

use anyhow;
use hyper::header::HOST;
use scraper::{Html, Selector};

use database_handler::models::{all_filings};
use request_handler::{QueueRequest, RequestData};

use crate::{Insider};
use crate::CallbackError;

#[derive(Clone, Debug)]
struct IndexReqData {
    pub base_url: String,
    pub accession_number: i64,
}

impl Insider {
    pub async fn run_index(&mut self) -> Result<(), CallbackError> {
        // Find unfulfilled requests in the db where old = false + make requests + insert to database
        self.index_make_request().await?;
        Ok(())
    }

    async fn index_make_request(&self) -> Result<(), CallbackError> {
        let index_ciks = all_filings::AllFilings::select_accession_index()?;

        let requests = self.index_create_requests(index_ciks);
        self.queue_request(requests,
                           self.config.sec.delay_milli,
                           self.config.sec.concurrent,
                           Insider::index_callback).await?;

        Ok(())
    }

    fn index_create_requests(&self, company_ciks: Vec<all_filings::AccessionIndex>) -> Vec<RequestData<IndexReqData>> {
        let mut headers = self.config.sec.headers.clone();
        headers.remove(HOST);
        let mut requests = Vec::new();
        for (i, filing) in company_ciks.iter().enumerate() {
            let request = RequestData {
                url: filing.index_link.as_ref().unwrap().to_string(),
                headers: headers.clone(),
                count: i,
                data: IndexReqData {
                    accession_number: filing.accession_number,
                    base_url: self.config.sec.base_url.clone()
                },
            };
            requests.push(request);
        }
        requests
    }

    fn index_callback(response_slice: Vec<u8>, request_data: RequestData<IndexReqData>)
                      -> Result<(), anyhow::Error>
    {
        let content = std::str::from_utf8(&*response_slice)?;
        let link = Insider::index_get_link(content)?;

        all_filings::AllFilings::update_form_link(
            request_data.data.accession_number,
            request_data.data.base_url.replace("{}", &*link))?;

        Ok(())
    }

    fn index_get_link(content: &str) -> Result<String, CallbackError> {
        fn unwrap_or_err<T>(stri: Option<T>) -> Result<T, CallbackError> {
            match stri {
                Some(v) => Ok(v),
                None => Err(CallbackError::OptionErr("Failed to unwrap".to_string()))
            }
        }

        let document = Html::parse_document(&content);
        let selector = Selector::parse("table.tableFile").unwrap();
        let table = unwrap_or_err(document.select(&selector).next())?;

        for (i, row) in table.select(& Selector::parse("tr").unwrap()).enumerate() {
            if i == 0 { continue; }
            for cell in row.select(&Selector::parse("td").unwrap()) {
                for child in cell.select(&Selector::parse("a").unwrap()) {
                    if unwrap_or_err(child.text().next())?.ends_with(".xml") {
                        return Ok(child.value().attr("href").unwrap().parse().unwrap());
                    }
                }
            };
        }
        Err(CallbackError::HtmlParseErr("Failed to find link".to_string()))
    }
}