#![feature(proc_macro_hygiene, decl_macro)]

use crate::db;
use diesel::prelude::*;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/cheeses")]
pub fn get_cheeses() -> &'static str {
    use db::schema::cheeses::dsl::{cheeses, pasteurised};
    let connection = db::conn::establish_connection();
    let results = cheeses.filter(pasteurised.eq(true))
        .limit(5)
        .load::<db::models::Cheese>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    let mut res: String;
    for cheese in results {
        res.push_str(cheese.name);
        res.push_str(cheese.milk);
    }
    &res
}