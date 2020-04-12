#![feature(proc_macro_hygiene, decl_macro)]

use crate::db;
use diesel::prelude::*;
use crate::db::schema;
use db::models::*;
use rocket::http::Status;
use rocket_contrib::json::Json;


extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::db::conn::CheesesDbConn;

use crate::db::models::{NewCheese, Cheese};
use rocket_contrib::databases::diesel;



pub fn create_cheese(conn: CheesesDbConn, name: &str, photo: &str) -> Cheese {
    use schema::cheeses;

    let new_cheese = NewCheese {
        name: name,
        photo: Option::Some(photo.to_owned()),
        milk: schema::Milk::Cow,
        pasteurised: true,
        cheesetype: schema::CheeseType::Blue,
        rind: schema::Rind::Na,
        country: schema::Country::England,
        additive: Option::None,
        region: Option::None,
        rating: Option::None,
        comment: Option::None,
        maturity: Option::None
    };

    diesel::insert_into(cheeses::table)
        .values(&new_cheese)
        .get_result(&*conn)
        .expect("Error saving new cheese")
}



// use rocket_contrib::databases::diesel as rockdiesel;
// #[database("cheeses")]
// pub struct CheesesDbConn(rockdiesel::PgConnection);

// #[get("/cheeses")]
// pub fn get_cheeses(conn: CheesesDbConn) -> Result<Json<Vec<Cheese>>, Status> {
//     use db::schema::cheeses::dsl::*;
//         cheeses.filter(name.eq("fop"))
//         .limit(5)
//         .load::<Cheese>(&*conn)
//         .map(|chss| Json(chss))
//         .map_err(|error| Status::NotFound)
    
// }

#[post("/make_cheese")]
pub fn make_cheese(conn: CheesesDbConn) -> String {
    
    let mut name = "Some Title".to_owned();
    let mut photo = "Some Photo".to_owned();


    let cheese = create_cheese(conn, &name, &photo);
    name
}

#[get("/cheeses")]
pub fn get_cheeses(conn: CheesesDbConn) -> String {
    use db::schema::cheeses::dsl::{cheeses, pasteurised};
    use crate::db::schema::EnumToPrint;

    let results = cheeses.filter(pasteurised.eq(true))
        .limit(5)
        .load::<db::models::Cheese>(&*conn)
        .expect("Error loading cheeses");

    println!("Displaying {} cheeses", results.len());

    let mut res: String = String::from("");
    for cheese in results {
        res.push_str(&cheese.name);
        res.push_str(&cheese.milk.to_print());
    }
    res
}

