use crate::model::user::User;
use actix_web::{web, HttpResponse, http, Error, error};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use bson::{doc, Document};
use crate::constant;
use crate::model::response_body::ResponseBody;
use crate::model::jwt::JwtClaims;

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


pub fn register(
    user_form: web::Form<AuthUser>,
    db: web::Data<mongodb::Database>
) -> HttpResponse {
    let new_user = user_form.into_inner();
    let validate_ret: Result<(), ValidationErrors> = new_user.validate();
    match validate_ret {
        Ok(_) => (),
        Err(error) => {
            info!("Invalid content submitted: {}", error);
            return HttpResponse::BadRequest()
                    .json(ResponseBody::new("Invalid content submitted", error));
        }
    }

    let collection = db.collection(constant::MONGO_COLL_USER);
    let find_ret = collection.find_one(doc! {"username": new_user.username.clone()}, None);
    return match find_ret {
        Ok(ret) => {
            match ret {
                Some(result_doc) => {
                    HttpResponse::NotAcceptable()
                        .json(ResponseBody::new("User already exist", constant::EMPTY))
                },
                None => {
                    let user = User {
                        id: bson::oid::ObjectId::new().unwrap(),
                        username: new_user.username.clone(),
                        email: new_user.email.clone(),
                        password: User::set_password(new_user.password.as_ref()),
                        realname: None,
                        bio: None
                    };

                    let user_bson = bson::to_bson(&user).unwrap();
                    if let bson::Bson::Document(document) = user_bson {
                        let ret = collection.insert_one(document, None);
                        match ret {
                            Ok(inserted) => {
                                HttpResponse::Ok()
                                    .json(ResponseBody::new("User created", constant::EMPTY))
                            },
                            Err(error) => {
                                error!("Failed when saving user to database: {}", error);
                                HttpResponse::InternalServerError()
                                    .json(ResponseBody::new("Failed when saving user to database", constant::EMPTY))
                            }
                        }
                    } else {
                        HttpResponse::InternalServerError()
                            .json(ResponseBody::new("Failed to serialize user", constant::EMPTY))
                    }
                }
            }
        },
        Err(error ) => {
            error!("Failed to check user existence: {}", error);
            HttpResponse::InternalServerError()
                .json(ResponseBody::new("Failed to check user existence", constant::EMPTY))
        }
    }
}

pub async fn login(
    user_form: web::Form<AuthUser>,
    db: web::Data<mongodb::Database>
) -> HttpResponse {
    let login_user = user_form.into_inner();
    let validate_ret = login_user.validate();
    match validate_ret {
        Ok(_) => (),
        Err(error) => {
            return HttpResponse::BadRequest()
                .json(ResponseBody::new("Invalid content submitted", error));
        }
    }

    let collection = db.collection(constant::MONGO_COLL_USER);
    let find_ret = collection.find_one(doc! {"username": login_user.username}, None);
    return match find_ret {
        Ok(ret) => {
            match ret {
                Some(result_doc) => {
                    match bson::from_bson(bson::Bson::Document(result_doc)) {
                        Ok(result) => {
                            let user: User = result;
                            if user.compare_password(login_user.password.as_str()) {
                                let claims = JwtClaims::new(&user);
                                HttpResponse::Ok()
                                    .json(ResponseBody::new("Authorised", claims.generate_token()))
                            } else {
                                HttpResponse::NotFound()
                                    .json(ResponseBody::new("User not found", ""))
                            }
                        },
                        Err(error) => {
                            HttpResponse::InternalServerError()
                                .json(ResponseBody::new("Invalid user returned from database", error.to_string()))
                        }
                    }
                },
                None => {
                    HttpResponse::NotFound()
                        .json(ResponseBody::new("User not found", constant::EMPTY))
                }
            }
        },
        Err(error) => {
            HttpResponse::InternalServerError()
                .json(ResponseBody::new("Failed to find user", constant::EMPTY))
        }
    }
}