#[macro_use]
extern crate log;

use std;
use std::fs;
use thiserror::Error;
use tokio;
use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde_json;
use database_handler::database_errors::DbError;

use database_handler::models::stock_data::StockData;
use shared_lib::logger::{build_default_logger, Log};
use models::serde_models::StockDataVec;

mod json_requests;
mod xml_requests;
mod rss_requests;
mod index_requests;
mod settings;
mod models;


#[derive(Error, Debug)]
pub enum InsiderError {
    #[error("Error in database")]
    DbError(#[from] DbError),
    #[error("Io Error")]
    IoError(#[from] std::io::Error),
    #[error("Serde Error")]
    SerdeError(#[from] serde_json::Error),

    #[error("Internal Json Error")]
    JsonError(#[from] json_requests::JsonError),
}

pub enum CallbackState {
    JsonRecent,
    JsonSubsequent,
    Rss,
    Index,
    Xml
}

struct Insider {
    config: settings::Settings,
    callback_state: CallbackState,
    https_connector: Client<HttpsConnector<HttpConnector>>
}

impl Insider {
    fn init(config: settings::Settings) -> Insider {
        Insider {
            config,
            https_connector: Client::builder().build::<_, hyper::Body>(HttpsConnector::new()),
            callback_state: CallbackState::JsonRecent,
        }
    }

    fn insert_default(&self) -> Result<usize, InsiderError> {
        let json_sli = fs::read("scraper/src/default_stock_data.json")?;
        let json_objs: StockDataVec = serde_json::from_slice(&json_sli)?;
        let r = StockData::insert_many(json_objs.defaults)?;
        info!("Inserted {} default stock data records", r);
        Ok(r)
    }

    async fn run(&mut self) -> Result<(), InsiderError> {
        println!("Running insider");
        self.run_json().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), InsiderError> {
    let config = settings::Settings::default();

    let mut logger = build_default_logger(Log::default()).unwrap();
    logger.init();

    let mut insider = Insider::init(config);
    // insider.insert_default().expect("Failed to insert default records");
    insider.run().await?;

    Ok(())
}
