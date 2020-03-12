use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use crate::model::user::User;
use bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    uid: bson::oid::ObjectId,
    email: String,
}

impl JwtClaims {
    pub fn new(user: &User) -> Self {
        return JwtClaims {
            uid: user.id.clone(),
            email: user.email.clone()
        }
    }

    pub fn generate_token(&self) -> String {
        
    }

    pub fn validate_token(token: String) -> bool {

    }
}