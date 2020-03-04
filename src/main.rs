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

pub mod model;

use std::env;
use actix_web::{get, App, HttpServer, Responder, web};
use r2d2_mongodb::{MongodbConnectionManager, ConnectionOptions};
use r2d2::Pool;


#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let dbManager = MongodbConnectionManager::new(
    ConnectionOptions::builder()
                .with_host(env::var("MONGO_HOST").unwrap().as_str(),
                           env::var("MONGO_PORT").unwrap().parse::<u16>().unwrap())
                .with_db(env::var("MONGO_DBNAME").unwrap().as_str()).build()

    );

    let pool = Pool::builder().max_size(10).build(manager).unwrap();

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
