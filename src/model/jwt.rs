use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use crate::model::user::User;
use bson::oid::ObjectId;
use std::env;
use crate::common::constant;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
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
        return encode(&Header::default(), self, &EncodingKey::from_secret(env::var(constant::JWT_SECRET).unwrap().as_bytes().clone())).unwrap();
    }

    pub fn is_token_valid(token: &str) -> bool {
        let token = decode::<JwtClaims>(token,
                                        &DecodingKey::from_secret(
                                            env::var(constant::JWT_SECRET).unwrap().as_bytes()),
                                        &Validation::default()
        );
        return match token {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub async fn token_validator(
        req: ServiceRequest,
        credentials: BearerAuth
    ) -> Result<ServiceRequest, Error> {
        if Self::is_token_valid(credentials.token()) {
            Ok(req)
        } else {
            let config = req.app_data::<Config>()
                .map(|data| data.get_ref().clone())
                .unwrap_or_else(Default::default)
                .scope("");
            Err(AuthenticationError::from(config).into())
        }
    }
}