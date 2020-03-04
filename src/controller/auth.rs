use actix_web::{web, HttpResponse, http, Error};
use r2d2::Pool;
use r2d2_mongodb::MongodbConnectionManager;
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthUser {
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String,
    #[serde(default)]
    email: String,
}

pub async fn register(
    user: web::Form<AuthUser>,
    pool: web::Data<Pool<MongodbConnectionManager>>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {

}

pub async fn login(
    user: web::Query<AuthUser>,
    pool: web::Data<Pool<MongodbConnectionManager>>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {

}