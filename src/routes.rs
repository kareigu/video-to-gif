use actix_web::{web::get, Result, HttpRequest, Route};
use actix_files::{NamedFile, Files};

async fn index(_req: HttpRequest) -> Result<NamedFile> {
  Ok(NamedFile::open("front/build/index.html")?)
}

pub fn index_route() -> Route {
  get().to(index)
}

pub fn index_files() -> Files {
  Files::new("/", "front/build")
}
