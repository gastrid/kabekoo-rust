#![feature(proc_macro_hygiene, decl_macro)]

use crate::db;
use diesel::prelude::*;
use crate::db::schema;
use db::models::*;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket::request::LenientForm;
use rocket::http::RawStr;
use rocket::request::FromFormValue;


extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::db::conn::CheesesDbConn;

use crate::db::models::{NewCheese, Cheese};
use rocket_contrib::databases::diesel;



#[post("/make_cheese", format = "application/x-www-form-urlencoded", data = "<cheese>")]
pub fn make_cheese(cheese: LenientForm<TestCheese>, conn: CheesesDbConn) -> String {
    use schema::cheeses;
    
    println!("{:?}", cheese.0);

    let smth = cheese.into_inner();
    smth.name

    // let new_cheese = NewCheese{
    // name: &*smth.name,
    // photo: smth.photo,
    // milk: smth.milk,
    // pasteurised: smth.pasteurised,
    // cheesetype: smth.cheesetype,
    // rind: smth.rind,
    // additive: smth.additive,
    // region: smth.region,
    // country: smth.country,
    // rating: smth.rating,
    // comment: smth.comment,
    // maturity: smth.maturity,
    // };

    // diesel::insert_into(cheeses::table)
    //     .values(&new_cheese)
    //     .get_result::<Cheese>(&*conn)
    //     .expect("Error saving new cheese");
    // TODO: add wiki photo stuffc
    // String::from("Hello")
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




#[derive(FromForm, Debug)]
pub struct TestCheese {
    pub name: String,

}