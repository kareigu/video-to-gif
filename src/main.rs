use actix_web::{get, App, HttpResponse, HttpServer, Responder};

static PORT: u16 = 8080;
static WORKERS: usize = 4;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .workers(WORKERS)
    .bind(format!("127.0.0.1:{}", PORT))?
    .run()
    .await
}