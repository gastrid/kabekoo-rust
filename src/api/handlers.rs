#![feature(proc_macro_hygiene, decl_macro)]

use crate::db;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/cheeses")]
pub fn cheeses() -> &'static str {
    let connection = db::establish_connection();
    let results = db::cheeses.filter()
        .limit(5)
        .load::<db::models::Cheese>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    let mut res: String;
    for cheese in results {
        res.push_str(cheese.name);
        res.push_str(cheese.milk);
    }
    res
}