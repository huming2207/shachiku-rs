use bson;

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


