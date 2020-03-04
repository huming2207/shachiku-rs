use bson;

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String
}

impl User {
    pub fn find_user_with_username(username: &str, conn: &dyn Connection) -> QueryResult<User> {

    }
}

