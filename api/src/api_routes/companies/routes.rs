use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::ServiceConfig;
use serde::Deserialize;

use database_handler::models::stock_data::{CompaniesParams, StockData};
use crate::api_errors::ApiError;


#[get("/data/")]
async fn data(params: web::Query<CompaniesParams>) -> Result<HttpResponse, ApiError> {
    let resp = match StockData::select_parameters(params.into_inner()) {
        Ok(r) => r,
        Err(e) => return Err(ApiError::from(e))
    };

    Ok(HttpResponse::Ok().json(resp))
}


pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(data);
}