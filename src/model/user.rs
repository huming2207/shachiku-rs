#[macro_use]
extern crate diesel;

use std::time::SystemTime;

#[cfg(test)]
use diesel::debug_query;
#[cfg(test)]
use diesel::pg::Pg;
use diesel::prelude::*;


table! {
    users {
        id -> Integer,
        username -> Text,
        email -> Text,
        password -> Text,
        bio -> Text
    }
}

#[derive(Queryable, Identifiable, AsChangeset)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: String
}

impl User {
    pub fn find_user_with_username(username: &str, conn: &dyn Connection) -> QueryResult<User> {
    }
}

