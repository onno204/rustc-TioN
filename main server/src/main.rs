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
        ])
        .mount("/api/user", routes![
            public::user::info_post
        ])
        .mount("/api/management/device", routes![
            public::management::device::add_device,
        ])
        .mount("/api/management/securitypool", routes![
            public::management::securitypool::create_post,
        ])

        .register(catchers![error_not_found, error_unprocessable_enitity, error_unauthorized])
    .launch();
}

#[catch(404)]
fn error_not_found(req: &rocket::Request) -> rocket::response::content::Json<String> {
    println!("req: {}", req);
    let request_types: Vec<_> = req.headers().get("Content-Type").collect();
    if request_types.len() >= 1 {
        let _request_type: &str = request_types[0];
        if _request_type != "application/json" {
            return rocket::response::content::Json(json!({
                "success": false,
                "error": "file_not_found",
                "message": "Try creating a POST 'application/json' request"
            }).to_string());
        }
    }
    return rocket::response::content::Json(json!({
        "success": false,
        "error": "file_not_found"
    }).to_string());

}

#[catch(401)]
fn error_unauthorized(_req: &rocket::Request) -> rocket::response::content::Json<String> {
    return rocket::response::content::Json(json!({
        "success": false,
        "error": "Unauthorized"
    }).to_string());
}

#[catch(500)]
fn error_unprocessable_enitity(_req: &rocket::Request) -> rocket::response::content::Json<String> {
    return rocket::response::content::Json(json!({
        "success": false,
        "error": "internal_server_error"
    }).to_string());
}
