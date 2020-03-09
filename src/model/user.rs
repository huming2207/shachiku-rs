use bson;
use serde::{Deserialize, Serialize};
use argon2::{self, Config};
use std::env;
use crate::constant;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<bson::oid::ObjectId>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub realname: Option<String>,
    pub bio: Option<String>,
}

impl User {
    pub fn set_password(raw_passwd: &str) -> String {
        let salt = env::var(constant::PASSWORD_SALT).unwrap();
        let config = Config::default();
        return argon2::hash_encoded(raw_passwd.as_ref(), salt.as_ref(), &config).unwrap();
    }

    pub fn compare_password(&self, raw_passwd: &str) -> bool {
        return argon2::verify_encoded(self.password.as_str(), raw_passwd.as_ref()).unwrap();
    }
}