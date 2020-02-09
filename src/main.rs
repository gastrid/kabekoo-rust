#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod api;
mod db;

fn main() {
    rocket::ignite().mount("/", routes![
        api::handlers::index,
        api::handlers::cheeses
        ]).launch();
}