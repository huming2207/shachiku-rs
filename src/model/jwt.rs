use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use crate::model::user::User;
use bson::oid::ObjectId;
use std::env;
use crate::common::constant;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web::Error;
use actix_web_httpauth::extractors::AuthenticationError;

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
        let secret = env::var(constant::JWT_SECRET).unwrap().as_bytes();
        return encode(&Header::default(), self, &EncodingKey::from_secret(secret))?;
    }

    pub fn is_token_valid(token: &str) -> bool {
        let secret = env::var(constant::JWT_SECRET).unwrap().as_bytes();
        let token = decode::<JwtClaims>(token, &DecodingKey::from_secret(secret), &Validation::default());
        return match token {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub async fn token_validator(
        req: ServiceRequest,
        credentials: BearerAuth
    ) -> Result<ServiceRequest, Error> {
        if is_token_valid(credentials.token()) {
            Ok(req)
        } else {
            Err(AuthenticationError::new("Unauthorised request, invalid JWT provided"));
        }
    }
}