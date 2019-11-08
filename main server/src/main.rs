#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate serde_json;
#[macro_use] extern crate rocket;
mod public;
mod server;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            public::index::get,
            public::login::post
        ])
    .launch();
}
