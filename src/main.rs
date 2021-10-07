#[macro_use] extern crate actix_web;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

mod schema;
mod db;
mod user;
mod controller;

use std::env;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use crate::db::establish_connection;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(controller::init_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}