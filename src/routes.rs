use actix_web::{get, HttpResponse};
use actix_files::{Files};
use std::fs;


#[get("/")]
pub fn index_route() -> HttpResponse {
  let index = fs::read_to_string("front/build/index.html").unwrap();
  HttpResponse::Ok()
    .content_type("text/html")
    .header("Cross-Origin-Opener-Policy", "same-origin")
    .header("Cross-Origin-Embedder-Policy", "require-corp")
    .body(index)
}

pub fn index_files() -> Files {
  Files::new("/", "front/build")
}
