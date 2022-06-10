#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

use listenfd::ListenFd;
use actix_web::{App, HttpServer, middleware::Logger, web};

use shared_lib::logger::build_default_logger;

mod settings;
mod api_routes;
mod database;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = settings::Settings::default();
    let mut logger = build_default_logger(config.log).unwrap();
    logger.init();

    info!("Config file loaded");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("[%a] [%r] [Code %s] [%Dms]"))
            .service(web::scope("/api").configure(api_routes::init_routes))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind((config.actix_config.url, config.actix_config.port))?
    };

    server.run().await
}
