#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_json;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

mod public;
mod server;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            public::index::get,
            public::login::post
        ])
        .register(catchers![not_found])
    .launch();
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> rocket::response::content::Json<String> {
    println!("req: {}", req);
    return rocket::response::content::Json(json!({
        "success": false,
        "error": format!("{}", public::responses::Errors::FileNotFound)
    }).to_string());
}
