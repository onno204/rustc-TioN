#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
mod public;


fn main() {
    rocket::ignite()
        .mount("/", routes![public::index::get, public::login::get])
    .launch();
}
