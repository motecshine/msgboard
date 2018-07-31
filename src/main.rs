#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json};
#[derive(Serialize)]
struct Response { 
    code: u8,
    msg: String
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
        msg: "操作成功".to_string(),
    })
}

#[put("/disagree/<id>")]
fn disagree(id: u8) -> Json<Response> {
    println!("{}", &id);
    Json(Response{
        code: 0,
        msg: "操作成功".to_string(),
    })
}

#[post("/new")]
fn new(id: u8) -> Json<Response> {
    println!("{}", &id);
    Json(Response{
        code: 0,
        msg: "操作成功".to_string(),
    })
}

#[catch(404)]
fn not_found() -> Json<Response> {
    Json(Response{
         code: 1,
         msg: "页面没找到".to_string(),
    })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).catch(catchers![not_found]).launch();
    rocket::ignite().mount("/post", routes![agree, disagree, new]).catch(catchers![not_found]).launch();
}