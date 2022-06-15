use std::fmt::{Debug, Error};
use std::fs;
use std::process::exit;
use std::time::Instant;

use anyhow;
use async_trait::async_trait;
use hyper::{Client, HeaderMap};
use hyper::client::HttpConnector;
use hyper::header::HOST;
use hyper_tls::HttpsConnector;
use quick_xml;
use scraper::{Html, Selector};

use database_handler::database_errors::DbError;
use database_handler::models::{all_filings, json_docs, stock_data, filings_data, all_insiders, insider_roles};
use request_handler::{QueueRequest, RequestData};

use crate::{Insider, InsiderError};
use crate::CallbackError;
use crate::models::xml_response;

#[derive(Clone, Debug)]
struct XmlReqData {
    pub base_url: String,
    pub accession_number: i64,
    pub company_cik: i32
}

impl Insider {
    pub async fn run_xml(&mut self) -> Result<(), CallbackError> {
        // Find unfulfilled requests in the db where old = false + make requests + insert to database
        self.xml_make_request().await?;
        Ok(())
    }

    async fn xml_make_request(&self) -> Result<(), CallbackError> {
        let form_acs = all_filings::AllFilings::select_accession_form()?;

        let requests = self.xml_create_requests(form_acs);

        self.queue_request(requests,
                           self.config.sec.delay_milli,
                           self.config.sec.concurrent,
                           Insider::xml_callback).await?;

        Ok(())
    }

    fn xml_create_requests(&self, company_ciks: Vec<all_filings::AccessionForm>) -> Vec<RequestData<XmlReqData>> {
        let mut headers = self.config.sec.headers.clone();
        headers.remove(HOST);
        let mut requests = Vec::new();
        for (i, filing) in company_ciks.iter().enumerate() {
            let request = RequestData {
                url: filing.form_link.as_ref().unwrap().to_string(),
                headers: headers.clone(),
                count: i,
                data: XmlReqData {
                    accession_number: filing.accession_number,
                    base_url: self.config.sec.base_url.clone(),
                    company_cik: filing.company_cik
                },
            };
            requests.push(request);
        }
        requests
    }

    fn xml_callback(response_slice: Vec<u8>, request_data: RequestData<XmlReqData>)
                      -> Result<(), anyhow::Error>
    {
        fn parse_owners(owners: Vec<xml_response::ReportingOwner>, company_cik: i32)
                        -> Result<(Vec<all_insiders::AllInsiders>, Vec<insider_roles::InsiderRoles>), anyhow::Error>
        {
            let mut insiders = Vec::new();
            let mut roles = Vec::new();
            
            for owner in owners {
                if all_insiders::AllInsiders::exists(owner.info.insider_cik)? == false {
                    insiders.push( all_insiders::AllInsiders{
                        insider_cik: owner.info.insider_cik,
                        name: owner.info.name
                    });
                };
                
                if insider_roles::InsiderRoles::exists(owner.info.insider_cik, company_cik)? == false {
                    roles.push(insider_roles::InsiderRoles{
                        id: None,
                        insider_cik: owner.info.insider_cik,
                        company_cik,
                        director: owner.relationship.is_director,
                        officer: owner.relationship.is_officer,
                        ten_percent: owner.relationship.is_ten_percent_owner,
                        other: owner.relationship.is_other,
                        officer_title: owner.relationship.officer_title,
                        other_text: owner.relationship.other_text,
                        str1: owner.address.street1,
                        str2: owner.address.street2,
                        city: owner.address.city,
                        state: owner.address.state,
                        zip: owner.address.zip,
                        state_description: owner.address.state_description
                    });
                }
            }
            
            Ok((insiders, roles))
        }
        fn parse_derivatives(derivatives: Option<xml_response::DerivativeTable>)
            -> Result<(Vec<filings_data::DHolding>, Vec<filings_data::DTransaction>), anyhow::Error>
        {
            let mut d_holdings = Vec::new();
            let mut d_transactions = Vec::new();

            let derivatives = match derivatives {
                Some(d) => {
                    match d.values {
                        Some(v) => v,
                        None => return Ok((d_holdings, d_transactions))
                    }
                }
                None => return Ok((d_holdings, d_transactions))
            };
            for derivative in derivatives {
                match derivative {
                    xml_response::DerivativeVar::DerivativeHolding(holding) => {
                        d_holdings.push( filings_data::DHolding {
                            security_title: holding.security_title.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            price: holding.conversion_or_exercise_price.value,
                            exercise_date: holding.exercise_date.value,
                            expiration_date: holding.expiration_date.value,
                            underlying_security_title: holding.underlying_security.security_title.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            underlying_security_price: holding.underlying_security.share_price.value,
                            ownership_nature: holding.ownership_nature.direct_or_indirect_ownership.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            indirect_relation: holding.ownership_nature.nature_of_ownership
                                .unwrap_or_default().value
                        })
                    }
                    xml_response::DerivativeVar::DerivativeTransaction(transaction) => {
                        d_transactions.push( filings_data::DTransaction {
                            security_title: transaction.security_title.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            price: transaction.conversion_or_exercise_price.value,
                            transaction_date: transaction.transaction_date.value,
                            deemed_execution_date: transaction.deemed_execution_date
                                .unwrap_or_default().value,
                            transaction_code: transaction.transaction_coding.code,
                            transaction_equity_swap: transaction.transaction_coding.equity_swap_involved,
                            transaction_shares: transaction.transaction_amounts.transaction_shares.value,
                            transaction_share_price: transaction.transaction_amounts.transaction_price_per_share.value,
                            transaction_ad_code: transaction.transaction_amounts.transaction_acquired_disposed_code.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            underlying_security_title: transaction.underlying_security.security_title.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            underlying_security_price: transaction.underlying_security.share_price.value,
                            post_transaction_shares: transaction.post_transaction_amounts.shares_owned_following_transaction
                                .unwrap_or_default().value,
                            exercise_date: transaction.exercise_date.value,
                            expiration_date: transaction.expiration_date.value,
                            ownership_nature: transaction.ownership_nature.direct_or_indirect_ownership.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            indirect_relation: transaction.ownership_nature.nature_of_ownership
                                .unwrap_or_default().value
                        })
                    }
                }
            }
            Ok((d_holdings, d_transactions))
        }

        fn parse_non_derivatives(non_derivatives: Option<xml_response::NonDerivativeTable>)
                             -> Result<(Vec<filings_data::NdHolding>, Vec<filings_data::NdTransaction>), anyhow::Error>
        {
            let mut nd_holdings = Vec::new();
            let mut nd_transactions = Vec::new();

            let non_derivatives = match non_derivatives {
                Some(d) => {
                    match d.values {
                        Some(v) => v,
                        None => return Ok((nd_holdings, nd_transactions))
                    }
                }
                None => return Ok((nd_holdings, nd_transactions))
            };

            for non_derivative in non_derivatives {
                match non_derivative {
                    xml_response::NonDerivativeVar::NonDerivativeHolding(holding) => {
                        nd_holdings.push(filings_data::NdHolding {
                            security_title: holding.security_title.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            post_transaction_shares: holding.post_transaction_amounts.shares_owned_following_transaction
                                .unwrap_or_default().value,
                            ownership_nature: holding.ownership_nature.direct_or_indirect_ownership.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            indirect_relation: holding.ownership_nature.nature_of_ownership
                                .unwrap_or_default().value
                        })
                    }
                    xml_response::NonDerivativeVar::NonDerivativeTransaction(transaction) => {
                        nd_transactions.push( filings_data::NdTransaction {
                            security_title: transaction.security_title.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            transaction_date: transaction.transaction_date.value,
                            transaction_code: transaction.transaction_coding.code,
                            transaction_equity_swap: transaction.transaction_coding.equity_swap_involved,
                            transaction_shares: transaction.transaction_amounts.transaction_shares.value,
                            transaction_share_price: transaction.transaction_amounts.transaction_price_per_share.value,
                            transaction_ad_code: transaction.transaction_amounts.transaction_acquired_disposed_code.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            post_transaction_shares: transaction.post_transaction_amounts.shares_owned_following_transaction
                                .unwrap_or_default().value,
                            ownership_nature: transaction.ownership_nature.direct_or_indirect_ownership.value
                                .ok_or(CallbackError::UnwrapErr("unwrap fail".parse()?))?,
                            indirect_relation: transaction.ownership_nature.nature_of_ownership
                                .unwrap_or_default().value
                        })
                    }
                }
            }

            Ok((nd_holdings, nd_transactions))
        }

        fn parse_footnotes(footnotes_data: Option<Vec<xml_response::Footnotes>>)
                                 -> Result<Vec<filings_data::Footnote>, anyhow::Error>
        {
            let mut footnotes = Vec::new();

            let footnotes_data = match footnotes_data {
                Some(d) => d,
                None => return Ok(footnotes)
            };
            for footnotes_option in footnotes_data {
                match footnotes_option.footnote {
                    None => (),
                    Some(v) => {
                        for footnote in v {
                            footnotes.push( filings_data::Footnote {
                                footnote_id: footnote.id,
                                text: footnote.text
                            })
                        }
                    }
                }
            }

            Ok(footnotes)
        }

        let parsed: xml_response::Response = quick_xml::de::from_slice(&response_slice)?;

        
        let (insiders, roles) = parse_owners(parsed.reporting_owners, request_data.data.company_cik)?;
        let (d_holdings, d_transactions) = parse_derivatives(parsed.derivative_table)?;
        let (nd_holdings,nd_transactions) = parse_non_derivatives(parsed.non_derivative_table)?;
        let footnotes = parse_footnotes(parsed.footnotes)?;


        let d_holdings: serde_json::Value = serde_json::to_value(&d_holdings)?;
        let nd_holdings: serde_json::Value = serde_json::to_value(&nd_holdings)?;
        let d_transactions: serde_json::Value = serde_json::to_value(&d_transactions)?;
        let nd_transactions: serde_json::Value = serde_json::to_value(&nd_transactions)?;
        let footnotes: serde_json::Value = serde_json::to_value(&footnotes)?;

        let documents_insert = filings_data::FilingsData {
            accession_number: request_data.data.accession_number,
            d_holdings,
            d_transactions,
            nd_holdings,
            nd_transactions,
            footnotes
        };

        // This is a transactional operation which affects 3 different tables
        filings_data::FilingsData::insert(documents_insert,
                                          insiders,
                                          roles,
                                          request_data.data.accession_number)?;
        Ok(())
    }
}