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
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub uid: bson::oid::ObjectId,
    pub username: String,
    pub email: String,
    pub created: DateTime<Utc>,
    pub expires: DateTime<Utc>
}

impl JwtClaims {
    pub fn new(user: &User) -> Self {
        let expire_minutes = env::var(constant::JWT_EXPIRE_MIN).unwrap().parse::<i64>().unwrap();
        return JwtClaims {
            uid: user.id.clone(),
            username: user.email.clone(),
            email: user.email.clone(),
            created: Utc::now(),
            expires: Utc::now() + Duration::minutes(expire_minutes),
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
            Ok(claim) => claim.claims.expires < Utc::now(),
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