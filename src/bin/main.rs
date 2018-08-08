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

static mut DB: Option<diesel::MysqlConnection> = None;

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
   unsafe {
        match DB {
            Some(ref mut conn) => match update_agree(&conn, &id) {
                Ok(_) => Json(Response {
                    code: 0,
                    msg: "ok".to_string(),
                }),
                Err(err) => Json(Response {
                    code: 0,
                    msg: err.to_string(),
                }),
            },
            None => Json(Response {
                code: 0,
                msg: "db error".to_string(),
            }),
        }
    }
}

#[put("/disagree/<id>")]
fn disagree(id: i32) -> Json<Response> {
    unsafe {
        match DB {
            Some(ref mut conn) => match update_disagree(&conn, &id) {
                Ok(_) => Json(Response {
                    code: 0,
                    msg: "ok".to_string(),
                }),
                Err(err) => Json(Response {
                    code: 0,
                    msg: err.to_string(),
                }),
            },
            None => Json(Response {
                code: 0,
                msg: "db error".to_string(),
            }),
        }
    }
}

#[post("/new", data = "<new>")]
fn new<'r>(new: Form<NewPostsForm>) -> Json<Response> {
    unsafe {
        match DB {
            Some(ref mut conn) => match create_post(&conn, &new.get().content, &new.get().types) {
                Ok(_) => Json(Response {
                    code: 0,
                    msg: "ok".to_string(),
                }),
                Err(err) => Json(Response {
                    code: 0,
                    msg: err.to_string(),
                }),
            },
            None => Json(Response {
                code: 0,
                msg: "db error".to_string(),
            }),
        }
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
    unsafe {
        DB = Some(establish_connection());
    }
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/post", routes![agree, disagree, new])
        .catch(catchers![not_found])
        .launch();
}
