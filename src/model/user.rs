use bson;
use serde::{Deserialize, Serialize};
use argon2::{self, Config};
use std::env;
use crate::constant;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub realname: String,
    pub bio: String,
}

impl User {
    pub fn set_password(&mut self, raw_passwd: &str) {
        let salt = env::var(constant::PASSWORD_SALT).unwrap().as_str();
        let config = Config::default();
        self.password = argon2::hash_encoded(raw_passwd, salt, &config).unwrap();
    }

    pub fn compare_password(&mut self, raw_passwd: &str) -> bool {
        let salt = env::var(constant::PASSWORD_SALT).unwrap().as_str();
        let config = Config::default();
        return argon2::verify_encoded(self.password.to_str(), raw_passwd).unwrap();
    }
}