extern crate env_logger;
use actix_web::{App, HttpServer, middleware::Logger};
mod config;
mod routes;
mod certs;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = config::read_config();
    let builder = certs::read_certs();

    HttpServer::new(|| {
        App::new()
            .route("/", routes::index_route())
            .service(routes::index_files())
            .wrap(Logger::default())
    })
    .shutdown_timeout(config.shutdown_timeout)
    .workers(config.worker_count)
    .bind_openssl(format!("{}:{}", config.ip, config.port), builder)?
    .run()
    .await
}