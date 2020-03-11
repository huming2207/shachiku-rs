#[macro_use]
extern crate validator_derive;
extern crate validator;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate bson;
extern crate actix_rt;
extern crate serde;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate uuid;
extern crate argon2;

pub mod app;
pub mod model;
pub mod controller;
pub mod common;

use std::env;
use actix_web::{get, App, HttpServer, Responder, web};
use mongodb::{Client, options::ClientOptions};
use common::constant;
use actix_web::middleware::Logger;
use log::Level;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();
    let db_option = ClientOptions::parse(env::var(constant::MONGO_DB_HOST).unwrap().as_str()).unwrap();
    let db_client = Client::with_options(db_option).unwrap();
    let db_database = db_client.database(env::var(constant::MONGO_DB_NAME).unwrap().as_str());

    HttpServer::new(move || App::new()
        .wrap(Logger::default())
        .data(db_database.clone()).configure(app::load_services))
        .bind(env::var(constant::LISTEN_ADDR).unwrap().as_str())?
        .run()
        .await
}
