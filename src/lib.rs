#![feature(mpsc_select)]
#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewPost, Post};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &MysqlConnection, content: &str, types: &i32) -> Post {
    use schema::posts::dsl::{id, posts};
    let new_post = NewPost {
        content: content,
        types: types,
        agree: &0,
        disagree: &0,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
    posts.order(id.desc()).first(conn).unwrap()
}

pub fn update_agree(conn: &MysqlConnection, _id: &i32) -> diesel::QueryResult<usize> {
    use schema::posts::dsl::*;
    let query = diesel::update(posts.filter(id.eq(_id))).set(agree.eq(agree + 1));
    let debug_sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("debug:{}", debug_sql);
    query.execute(conn)
}

pub fn update_disagree(conn: &MysqlConnection, _id: &i32) -> diesel::QueryResult<usize> {
    use schema::posts::dsl::*;
    let query = diesel::update(posts.filter(id.eq(_id))).set(disagree.eq(disagree + 1));
    let debug_sql = diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string();
    println!("debug:{}", debug_sql);
    query.execute(conn)
}
