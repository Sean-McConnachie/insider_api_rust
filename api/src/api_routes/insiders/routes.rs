use actix_web::{get, HttpResponse};
use actix_web::web::ServiceConfig;

use database_handler::models::User;

use crate::api_errors::ApiError;

#[get("/")]
async fn index() -> Result<HttpResponse, ApiError> {
    let users = User::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}


pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(index);
}
