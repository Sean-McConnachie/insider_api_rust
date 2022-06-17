use actix_web::{get, HttpResponse, web};
use actix_web::web::ServiceConfig;

//use database_handler::models::User;
use database_handler::models::all_insiders::{AllInsiders, AllInsidersParams};
use database_handler::models::insider_roles::{InsiderRoles, InsidersRolesParams};
use crate::api_errors::ApiError;


#[get("/names/")]
async fn names(params: web::Query<AllInsidersParams>) -> Result<HttpResponse, ApiError> {
    let resp = match AllInsiders::select_parameters(params.into_inner()) {
        Ok(params) => params,
        Err(e) => return Err(ApiError::from(e))
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/roles/")]
async fn roles(params: web::Query<InsidersRolesParams>) -> Result<HttpResponse, ApiError> {
    let resp = match InsiderRoles::select_parameters(params.into_inner()) {
        Ok(params) => params,
        Err(e) => return Err(ApiError::from(e))
    };

    Ok(HttpResponse::Ok().json(resp))
}


pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(names)
        .service(roles);
}
