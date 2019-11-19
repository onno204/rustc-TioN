#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_json;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate mysql;
pub mod public;
pub mod server;
use std::env;


fn main() {
    // Development
    env::set_var("RUST_BACKTRACE", "1");
    rocket::ignite()
        .mount("/", routes![
            public::index::get,
            public::login::post
        ])
        .register(catchers![error_not_found, error_unprocessable_enitity])
    .launch();
}

#[catch(404)]
fn error_not_found(req: &rocket::Request) -> rocket::response::content::Json<String> {
    println!("req: {}", req);
    return rocket::response::content::Json(json!({
        "success": false,
        "error": format!("{}", public::responses::Errors::FileNotFound)
    }).to_string());
}

#[catch(500)]
fn error_unprocessable_enitity(req: &rocket::Request) -> rocket::response::content::Json<String> {
    println!("req: {}", req);
    return rocket::response::content::Json(json!({
        "success": false,
        "error": format!("{}", public::responses::Errors::InternalServerError)
    }).to_string());
}
