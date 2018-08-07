#![plugin(rocket_codegen)]
#![feature(plugin, custom_derive)]

extern crate diesel;
extern crate msgboard;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use msgboard::*;
use rocket::request::Form;
use rocket_contrib::Json;

#[derive(Serialize)]
struct Response {
    code: u8,
    msg: String,
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
fn agree(id: i32) -> Json<Response> {
    println!("{}", &id);
    let conn = establish_connection();
    let result = update_agree(&conn, &id);
    match result {
        Ok(_) => Json(Response {
            code: 0,
            msg: "ok".to_string(),
        }),
        Err(err) => Json(Response {
            code: 0,
            msg: err.to_string(),
        }),
    }
}

#[put("/disagree/<id>")]
fn disagree(id: i32) -> Json<Response> {
    let conn = establish_connection();
    let result = update_disagree(&conn, &id);
    match result {
        Ok(_) => Json(Response {
            code: 0,
            msg: "ok".to_string(),
        }),
        Err(err) => Json(Response {
            code: 0,
            msg: err.to_string(),
        }),
    }
}

#[post("/new", data = "<new>")]
fn new<'r>(new: Form<NewPostsForm>) -> Json<Response> {
    let conn = establish_connection();
    println!("{}", new.get().types);
    let result = create_post(&conn, &new.get().content, &new.get().types);
    match result {
        Ok(_) => Json(Response {
            code: 0,
            msg: "ok".to_string(),
        }),
        Err(err) => Json(Response {
            code: 0,
            msg: err.to_string(),
        }),
    }
}

#[catch(404)]
fn not_found() -> Json<Response> {
    Json(Response {
        code: 1,
        msg: "page not found!".to_string(),
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/post", routes![agree, disagree, new])
        .catch(catchers![not_found])
        .launch();
}
