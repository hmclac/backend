use actix_web::{
    middleware::{self, Logger},
    web, App, HttpResponse, HttpServer, Responder,
};
// use diesel::prelude::*;
// use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
// use std::env;

// type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod controllers;
pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::index_router::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
