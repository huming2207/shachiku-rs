#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate actix_rt;
extern crate serde;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate uuid;
extern crate bcrypt;

pub mod app;
pub mod model;
pub mod service;
pub mod controller;

use std::env;
use actix_web::{get, App, HttpServer, Responder, web};
use mongodb::{Client, options::ClientOptions};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_option = ClientOptions::parse(env::var("MONGO_DB_URI").unwrap().as_str()).unwrap();
    let db_client = Client::with_options(db_option).unwrap();
    let db_database = db_client.database(env::var("MONGO_DB_NAME").unwrap().as_str());

    HttpServer::new(|| App::new().data(db_database.clone()).configure(app::load_services))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
