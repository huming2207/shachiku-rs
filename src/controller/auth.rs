use actix_web::{web, HttpResponse, http, Error};
use mongodb::Database;
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
    db: web::Data<mongodb::Database>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {

}

pub async fn login(
    user: web::Query<AuthUser>,
    db: web::Data<mongodb::Database>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {

}