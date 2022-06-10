use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::ServiceConfig;

// Our main api endpoints
use crate::api_routes::companies;
use crate::api_routes::filings;
use crate::api_routes::insiders;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Index of /api/")
}


pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(index)
        .service(web::scope("/companies").configure(companies::init_routes))
        .service(web::scope("/filings").configure(filings::init_routes))
        .service(web::scope("/insiders").configure(insiders::init_routes));
}