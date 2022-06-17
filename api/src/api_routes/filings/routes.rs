use actix_web::{get, HttpResponse, web};
use actix_web::web::ServiceConfig;

use crate::api_errors::ApiError;

use database_handler::models::all_filings::{AllFilings, FilingParams, ParsedFilingParams};


#[get("/metadata/")]
async fn metadata(params: web::Query<FilingParams>) -> Result<HttpResponse, ApiError> {
    let parsed = ParsedFilingParams::try_from(params.into_inner())?;
    let resp = AllFilings::select_parameters_no_tables(parsed)?;
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/data/")]
async fn data(params: web::Query<FilingParams>) -> Result<HttpResponse, ApiError> {
    let parsed = ParsedFilingParams::try_from(params.into_inner())?;
    let resp = AllFilings::select_parameters_with_tables(parsed)?;
    Ok(HttpResponse::Ok().json(resp))
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(metadata)
        .service(data);
}