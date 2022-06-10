use actix_web::{HttpResponse, Responder, get};
use actix_web::web::ServiceConfig;


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Index of /filings/")
}


pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(index);
}