#![plugin(rocket_codegen)]
#![feature(plugin, custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


use rocket::request::{Form};
use rocket_contrib::{Json};

#[derive(Serialize)]
struct Response { 
    code: u8,
    msg: String
}

#[derive(Debug, FromForm)]
struct NewPostsForm {
    types: u8,
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
fn new<'r>(new: Result<Form<'r, NewPostsForm>, Option<String>>) -> Json<Response> {
    match new {
        Ok(form) => format!("{:?}", form.get()),
        Err(Some(f)) => format!("Invalid form input: {}", f),
        Err(None) => format!("Form input was invalid UTF8."),
    };
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