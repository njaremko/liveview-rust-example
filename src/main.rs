#[macro_use]
extern crate failure;

use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix_files as fs;

mod socket;
mod template;

pub type Result<T> = std::result::Result<T, failure::Error>;

fn main() -> Result<()> {
    HttpServer::new(|| App::new()
        .service(fs::Files::new("/static", "static/"))
        .service(fs::Files::new("/test", "static/html/").index_file("index.html"))
        .service(web::resource("/ws/").route(web::get().to(socket::start_socket)))
        .route("/", web::get().to(template::render)))
        .bind("127.0.0.1:8000")?
        .run()?;
    Ok(())
}
