#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod api;
mod config;
mod lib;
mod models;

use crate::models::node::Node;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use futures::io;
use futures::lock::Mutex;
use std::env;
use std::sync::Arc;

pub struct AppState {
    url: String,
    nodes: Arc<Mutex<Vec<Node>>>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let data = web::Data::new(AppState {
        url: db_url.clone(),
        nodes: Arc::new(Mutex::new(vec![])),
    });
    HttpServer::new(move || {
        App::new()
            // .wrap(
            //     Cors::default()
            //         .allowed_origin("http://127.0.0.1:3002")
            //         .send_wildcard()
            //         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            //         .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            //         .allowed_header(http::header::CONTENT_TYPE)
            //         .max_age(3600),
            // )
            .app_data(data)
            .wrap(actix_web::middleware::Logger::default())
            .configure(config::app::config_services)
    })
    .bind(&app_url)?
    .run()
    .await
}
