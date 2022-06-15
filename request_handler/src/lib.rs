#[macro_use]
extern crate log;

use std::any::Any;
use std::fmt::{Debug, Error};
use std::ops::Deref;
use async_trait::async_trait;
use hyper;
use hyper::body::HttpBody;
use hyper::{Client, HeaderMap, StatusCode};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use thiserror::Error;
use tokio::time::{Instant, sleep_until, Duration};

use futures::channel::oneshot;
use futures::channel::oneshot::{Receiver, Sender};
use futures::stream::{self, StreamExt};
use hyper::header::HeaderName;
use url::Url;
use anyhow;

pub type FetchErr = Box<dyn std::error::Error + Send + Sync>;

#[derive(Error, Debug)]
pub enum RequestHandlerError {
    #[error("Fetch error")]
    FetchErr(#[from] FetchErr),
    #[error("Header key error")]
    HeaderKeyErr(String),
    #[error("Status code error")]
    StatusCodeErr(u16),
    #[error("Hyper HTTP error")]
    HyperHttpErr(#[from] hyper::http::Error),
    #[error("Hyper request error")]
    HyperRequestErr(#[from] hyper::Error),
}

#[derive(Clone, Debug)]
pub struct RequestData<T> {
    pub url: String,
    pub headers: HeaderMap,
    pub count: usize,
    pub data: T
}

#[async_trait]
pub trait QueueRequest {
    fn get_client(&self) -> Client<HttpsConnector<HttpConnector>>;

    async fn queue_request<T>(&self,
                              requests: Vec<RequestData<T>>,
                              delay_milli: u16,
                              concurrent: u16,
                              callback: fn(Vec<u8>, RequestData<T>) -> Result<(), anyhow::Error>)
        -> Result<(), RequestHandlerError>
    where T: Debug + Clone + Send + Sync
    {
        let mut receivers = Vec::<Receiver<&RequestData<T>>>::new();
        let mut senders = Vec::<Sender<&RequestData<T>>>::new();
        for _ in 0..requests.len() {
            let (tx, rx) = oneshot::channel();
            receivers.push(rx);
            senders.push(tx);
        }

        let request_start_time = Instant::now();
        let delay_milli = delay_milli as usize;
        let t_requests = requests.len();

        let fut = stream::iter(receivers).for_each_concurrent(
        concurrent as usize,
            |rx| async move {

                let request_data: &RequestData<T> = rx.await.expect("Failed to unwrap request data");

                sleep_until(request_start_time + Duration::from_millis(
                    (request_data.count*delay_milli) as u64)).await;

                info!("[{}/{}] Making request for {}", request_data.count + 1, t_requests, request_data.url);
                let client = self.get_client();

                match fetch_buffer(&client, request_data).await {
                    Ok(r) => {
                        match callback(r, request_data.clone()) {
                            Ok(_) => debug!("Sucessfully completed operations for {}", request_data.url),
                            Err(_) => warn!("Failed to complete inserts for {}", request_data.url)
                        }
                    },
                    Err(e) => warn!("Failed to complete request for {}: {:?}", request_data.url, e)
                };

            },
        );

        let mut i = 0;
        for s in senders {
            s.send(&requests[i]).expect("Failed to send RequestData to future");
            i = i + 1;
        }

        fut.await;

        Ok(())
    }
}

pub async fn fetch_buffer<T>(client: &Client<HttpsConnector<HttpConnector>>,
                             request_data: &RequestData<T>) -> Result<Vec<u8>, RequestHandlerError> {

    let mut request = hyper::Request::get(&request_data.url)
        .body(hyper::body::Body::empty())?;

    for header in &request_data.headers {
        match header.0 {
            h => request.headers_mut().insert(h.clone(), header.1.clone()),
            _ => return Err(RequestHandlerError::HeaderKeyErr("Missing header key.".to_string()))
        };
    };

    let mut res = client.request(request).await?;
    let mut buf = Vec::<u8>::new();

    while let Some(next) = res.data().await {
        let chunk = next?;
        buf.extend(chunk);
    }

    // TODO: Error handling for status codes - i.e. Retry for network errors
    match res.status() {
        StatusCode::OK => Ok(buf),
        e => {
            println!("{:?}", std::str::from_utf8(&buf));
            Err(RequestHandlerError::StatusCodeErr(e.as_u16()))
        }
    }
}