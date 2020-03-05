use actix_web::{web, HttpResponse, http, Error};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, Validate)]
pub struct AuthUser {
    #[serde(default)]
    #[validate(length(min = 1))]
    username: String,
    #[serde(default)]
    #[validate(length(min = 5))]
    password: String,
    #[serde(default)]
    #[validate(email)]
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