use actix_web::{get, HttpResponse, Responder};
use actix_web::web::ServiceConfig;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Index of /companies/")
}


pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(index);
}