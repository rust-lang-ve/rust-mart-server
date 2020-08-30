#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate log;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpResponse, HttpServer};
use chrono::prelude::*;
use dotenv;
use std::env;

mod database;
mod error;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        dotenv::dotenv().ok().expect("Unable to read .env file");
        env::set_var("RUST_LOG", "warn,info,error,debug");
        env_logger::init();
        warn!("⚠️   Running in development mode");
    } else {
        env::set_var("RUST_LOG", "warn,error");
    }

    // initialize database pool
    // uncomment to initialize connection to the database
    // database::init();

    let target = format!("{}:{}", "0.0.0.0", "7878");

    println!("Server ready at {}", format!("http://{}", &target));

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT
                    ])
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            .service(routes::auth())
            .route(
                "/healthcheck",
                web::get().to(|| {
                    let utc_time: String = Utc::now().to_string();

                    HttpResponse::from(utc_time)
                }),
            )
    })
    .bind(target)?
    .run()
    .await
}
