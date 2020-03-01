#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_derive_enum;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;


#[macro_use] extern crate rocket_contrib;


mod api;
mod db;

fn main() {
    use api::handlers::CheesesDbConn;
    rocket::ignite()
        .attach(CheesesDbConn::fairing())
        .mount("/", routes![
        api::handlers::index,
        api::handlers::get_cheeses
        ]).launch();
}