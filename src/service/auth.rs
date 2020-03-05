use crate::controller::auth::AuthUser;
use actix_web::web;
use mongodb::Database;
use validator::Validate;
use validator_derive;

pub fn register(
    user: &AuthUser,
    db: web::Data<mongodb::Database>
) -> Result<mongodb::results::InsertManyResult, mongodb::error::Error> {
    user.s
    let result = user.validate(); // validate not found here
}