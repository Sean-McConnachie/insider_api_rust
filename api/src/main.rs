#[macro_use]
extern crate log;

use actix_web::{App, HttpServer, middleware::Logger, web};
use listenfd::ListenFd;

use shared_lib::logger::{build_default_logger, Log};

mod api_errors;
mod settings;
mod api_routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = settings::Settings::default();
    let mut logger = build_default_logger(Log::default()).unwrap();
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
