use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod config;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = config::read_config();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file("certs/key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("certs/cert.pem").unwrap();

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