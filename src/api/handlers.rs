

use crate::db;
use diesel::prelude::*;
use crate::db::schema;

use rocket_contrib::json::Json;


extern crate dotenv;

use crate::db::conn::CheesesDbConn;

use crate::db::models::{NewCheese, Cheese, FormCheese};
use rocket_contrib::databases::diesel;



#[post("/make_cheese", format = "application/json", data = "<cheese>")]
pub fn make_cheese(cheese: Json<FormCheese>, conn: CheesesDbConn) -> Json<Cheese> {
    use schema::cheeses;
    let inner_cheese = cheese.into_inner();
    

    let new_cheese = NewCheese{
    name: &*inner_cheese.name,
    photo: inner_cheese.photo,
    milk: inner_cheese.milk,
    pasteurised: inner_cheese.pasteurised,
    cheesetype: inner_cheese.cheesetype,
    rind: inner_cheese.rind,
    additive: inner_cheese.additive,
    region: inner_cheese.region,
    country: inner_cheese.country,
    rating: inner_cheese.rating,
    comment: inner_cheese.comment,
    maturity: inner_cheese.maturity,
    };

    let saved_cheese: Cheese = diesel::insert_into(cheeses::table)
        .values(&new_cheese)
        .get_result::<Cheese>(&*conn)
        .expect("Error saving new cheese");

    // TODO: add wiki photo stuffc
    // TODO: unwrap better result of get_result
    Json(saved_cheese)
}

#[get("/cheeses")]
pub fn get_cheeses(conn: CheesesDbConn) -> Json<Vec<Cheese>> {
    use db::schema::cheeses::dsl::{cheeses};

    let results = cheeses
        .load::<db::models::Cheese>(&*conn)
        .expect("Error loading cheeses");

    Json(results)
}

#[get("/cheese/<id>")]
pub fn get_by_id(id: i32, conn: CheesesDbConn) -> Json<Cheese> {
    use schema::cheeses;
    
    let result = cheeses::table.find(id).first::<db::models::Cheese>(&*conn).expect("Error loading cheese");

    Json(result)
}


