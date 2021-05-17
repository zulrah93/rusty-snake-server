#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod routes;
mod utils;

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::index::get])
        .launch();
}
