use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::ServiceConfig;

use crate::api_errors::ApiError;

use database_handler::models::all_filings::{AllFilings, FilingParams, ParsedFilingParams};


#[get("/metadata/")]
async fn metadata(params: web::Query<FilingParams>) -> Result<HttpResponse, ApiError> {
    /*
    let resp = match AllFilings::select_parameters(params.into_inner()) {
        Ok(params) => params,
        Err(e) => return Err(ApiError::from(e))
    };
     */
    println!("{:?}", params);
    let parsed = ParsedFilingParams::try_from(params.into_inner())?;
    println!("{:?}", parsed);
    Ok(HttpResponse::Ok().body(""))
}


pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(metadata);
}