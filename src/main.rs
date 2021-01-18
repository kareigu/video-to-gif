use actix_web::{App, HttpServer};
mod config;
mod routes;
mod certs;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = config::read_config();
    let builder = certs::read_certs();

    HttpServer::new(|| {
        App::new()
            .route("/", routes::index_route())
            .service(routes::index_files())
    })
    .shutdown_timeout(config.shutdown_timeout)
    .workers(config.worker_count)
    .bind_openssl(format!("127.0.0.1:{}", config.port), builder)?
    .run()
    .await
}