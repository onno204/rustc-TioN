#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_json;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate mysql;
#[macro_use] extern crate lazy_static;
pub mod public;
pub mod server;
// Development
// use std::env;


fn main() {
    // Development
    // env::set_var("RUST_BACKTRACE", "1");
    println!("Connecting to MySql server...");
    server::sql_connector::check_sql_pool().unwrap();
    println!("Connected");

    rocket::ignite()
        .mount("/", routes![
            public::index::get,
            public::login::post,
            public::register::post
        ])
        .mount("/api/securitypool", routes![
            public::securitypool::create_post
        ])
        .mount("/api/user", routes![
            public::user::info_post
        ])
        .register(catchers![error_not_found, error_unprocessable_enitity])
    .launch();
}

#[catch(404)]
fn error_not_found(req: &rocket::Request) -> rocket::response::content::Json<String> {
    println!("req: {}", req);
    return rocket::response::content::Json(json!({
        "success": false,
        "error": "file_not_found"
    }).to_string());
}

#[catch(500)]
fn error_unprocessable_enitity(req: &rocket::Request) -> rocket::response::content::Json<String> {
    println!("req: {}", req);
    return rocket::response::content::Json(json!({
        "success": false,
        "error": "internal_server_error"
    }).to_string());
}
