use bson;
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 5))]
    pub password: String,
    pub realname: String,
    pub bio: String,
}


