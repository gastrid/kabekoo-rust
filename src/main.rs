#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_derive_enum;
#[macro_use] extern crate diesel;

mod api;
mod db;

fn main() {
    rocket::ignite().mount("/", routes![
        api::handlers::index,
        api::handlers::get_cheeses
        ]).launch();
}