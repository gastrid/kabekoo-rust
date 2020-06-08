#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_derive_enum;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate percent_encoding;


#[macro_use] extern crate rocket_contrib;


mod api;
pub mod db;
pub mod parser;


fn main() {
    rocket::ignite()
        .attach(db::conn::CheesesDbConn::fairing())
        .mount("/", routes![
        api::handlers::make_cheese,
        api::handlers::get_cheeses,
        api::handlers::get_by_id,
        api::handlers::update_cheese,
        api::handlers::delete,
        ]).launch();
}