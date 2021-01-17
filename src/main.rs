use actix_web::{web, App, HttpServer, Result, HttpRequest};
use actix_files::NamedFile;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

static PORT: u16 = 8080;
static SHUTDOWN_TIME: u64 = 15;
static WORKERS: usize = 4;

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("front/build/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file("certs/key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("certs/cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/", "front/build"))
    })
    .shutdown_timeout(SHUTDOWN_TIME)
    .workers(WORKERS)
    .bind_openssl(format!("127.0.0.1:{}", PORT), builder)?
    .run()
    .await
}