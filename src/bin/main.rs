#![plugin(rocket_codegen)]
#![feature(plugin, custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
extern crate diesel;
extern crate msgboard;
#[macro_use] extern crate serde_derive;


use rocket::request::{Form};
use rocket_contrib::{Json};
use msgboard::*;

#[derive(Serialize)]
struct Response { 
    code: u8,
    msg: String
}

#[derive(Debug, FromForm)]
struct NewPostsForm {
    types: i32,
    content: String,
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[put("/agree/<id>")]
fn agree(id: u8) -> Json<Response> {
    println!("{}", &id);
    Json(Response{
        code: 0,
        msg: "success".to_string(),
    })
}

#[put("/disagree/<id>")]
fn disagree(id: u8) -> Json<Response> {
    println!("{}", &id);
    Json(Response{
        code: 0,
        msg: "success".to_string(),
    })
}

#[post("/new", data = "<new>")]
fn new<'r>(new: Form<NewPostsForm>) -> Json<Response> {
    let conn = establish_connection();
    println!("{}", new.get().types);
    create_post(&conn, &new.get().content, &new.get().types);
    Json(Response{
        code: 0,
        msg: "success".to_string(),
    })
}

#[catch(404)]
fn not_found() -> Json<Response> {
    Json(Response{
         code: 1,
         msg: "page not found!".to_string(),
    })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).
    mount("/post", routes![agree, disagree, new]).catch(catchers![not_found]).launch();
}