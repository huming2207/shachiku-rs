use crate::model::user::User;
use actix_web::web;
use mongodb::Database;

pub fn register(
    user: &User,
    db: web::Data<mongodb::Database>
) -> Result<mongodb::results::InsertManyResult, mongodb::error::Error> {

}