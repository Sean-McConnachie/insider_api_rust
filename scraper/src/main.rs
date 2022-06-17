#![allow(dead_code)] // TODO

#[macro_use]
extern crate log;
extern crate core;

use std;
use std::convert::Infallible;
use std::fs;
use std::process::exit;
use thiserror::Error;
use tokio;
use hyper::{Client, HeaderMap};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde_json;
use callbacks::json_requests;
use database_handler::database_errors::DbError;

use database_handler::models::stock_data::StockData;
use shared_lib::logger::{build_default_logger, Log};
use models::other_models::StockDataVec;
use request_handler::{QueueRequest, RequestHandlerError};

mod settings;
mod models;
mod callbacks;
mod custom_de_serializers;


#[derive(Error, Debug)]
pub enum InsiderError {
    #[error("Error in database")]
    DbError(#[from] DbError),
    #[error("Io Error")]
    IoError(#[from] std::io::Error),
    #[error("Serde Error")]
    SerdeError(#[from] serde_json::Error),
    #[error("Internal Callback Error")]
    CallbackErr(#[from] CallbackError),
}

#[derive(Error, Debug)]
pub enum CallbackError {
    #[error("Request handler error")]
    RequestHandlerErr(#[from] RequestHandlerError),
    #[error("Database error")]
    DatabaseErr(#[from] DbError),
    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
    #[error("HTML Parse error")]
    HtmlParseErr(String),
    #[error("Option error")]
    OptionErr(String),
    #[error("Anyhow, lets continue")]
    AnyhowErr(#[from] anyhow::Error),
    #[error("Unwrap error")]
    UnwrapErr(String),
}

struct Insider {
    config: settings::Settings,
    https_connector: Client<HttpsConnector<HttpConnector>>
}

impl Insider {
    fn init(config: settings::Settings) -> Insider {
        Insider {
            config,
            https_connector: Client::builder().build::<_, hyper::Body>(HttpsConnector::new()),
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
        // TODO: Make error catches for all callbacks insider their mod files (i.e. catch individual requests)
        info!("Starting json collection");
        match self.run_json().await {
            Ok(_) => warn!("Successfully ran json."),
            Err(e) => info!("Error running json: {:?}", e)
        }

        info!("Starting RSS collection.");
        match self.run_rss().await {
            Ok(_) => warn!("Successfully ran RSS."),
            Err(e) => info!("Error running RSS: {:?}", e)
        }

        info!("Starting index collection");
        match self.run_index().await {
            Ok(_) => warn!("Successfully ran index."),
            Err(e) => info!("Error running index: {:?}", e)
        }


        match self.run_xml().await {
            Ok(_) => warn!("Successfully ran xml."),
            Err(e) => info!("Error running xml: {:?}", e)
        }

        Ok(())
    }
}


impl QueueRequest for Insider {
    fn get_client(&self) -> Client<HttpsConnector<HttpConnector>> {
        self.https_connector.clone()
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
