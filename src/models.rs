use super::schema::posts;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: uuid::Uuid,
    pub time_created: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub author_id: uuid::Uuid,
    // time_created is handled by the database
}
