use std::fmt::Debug;
use crate::{Insider, InsiderError};

use thiserror::Error;
use hyper::{Client, HeaderMap};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use request_handler::{QueueRequest, RequestData};
use async_trait::async_trait;


#[derive(Error, Debug)]
pub enum JsonError {

}

#[derive(Clone, Debug)]
struct ExampleData {
    pub something: String
}


impl Insider {
    pub async fn run_json(&mut self) -> Result<(), JsonError> {
        let url = "https://dumbstockapi.com/stock?exchanges=NYSE".to_string();
        let requests = Insider::generate_requests(url);
        self.queue_request(requests,
                           self.config.sec.delay_milli,
                           self.config.sec.concurrent).await.unwrap();
        Ok(())
    }

    fn generate_requests(url: String) -> Vec<RequestData<ExampleData>> {
        let mut requests = Vec::new();
        for i in 0..52 {
            let r = RequestData {
                data: ExampleData { something: format!("Something {}", i) },
                url: url.clone(),
                headers: HeaderMap::default(),
                count: i
            };
            requests.push(r);
        }
        requests
    }
}

impl QueueRequest for Insider {
    fn get_client(&self) -> Client<HttpsConnector<HttpConnector>> {
        self.https_connector.clone()
    }

    fn request_complete<T>(&self, response_slice: &Vec<u8>, request_data: &RequestData<T>)
        where T: Debug + Clone
    {
        //let e  = ExampleData::try_from(request_data.data.clone()).unwrap();
        println!("{:?}", request_data);
    }
}