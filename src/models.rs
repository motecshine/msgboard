extern crate chrono;

use schema::posts;
use chrono::NaiveDateTime;


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub content: String,
    pub types: i32,
    pub agree: i32,
    pub disagree: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub content: &'a str,
    pub agree: &'a i32,
    pub disagree: &'a i32,
    pub types: &'a i32, 
}