

use crate::db;
use diesel::prelude::*;
use crate::db::schema;

use rocket_contrib::json::Json;


extern crate dotenv;

use crate::db::conn::CheesesDbConn;

use crate::db::models::{NewCheese, Cheese, FormCheese};
use rocket::request::{Form};
use schema::*;
use rocket_contrib::databases::diesel;


#[derive(FromForm)]
pub struct GetFilters {
    pub milk: Option<Milk>,
    pub pasteurised: Option<bool>,
    pub cheesetype: Option<CheeseType>,
    pub rind: Option<Rind>,
    pub country: Option<Country>,
}


#[post("/make_cheese", format = "application/json", data = "<cheese>")]
pub fn make_cheese(cheese: Json<FormCheese>, conn: CheesesDbConn) -> Json<Cheese> {
    let inner_cheese = cheese.into_inner();
    

    let new_cheese = NewCheese{
    name: inner_cheese.name,
    photo: inner_cheese.photo,
    milk: Option::Some(inner_cheese.milk),
    pasteurised: Option::Some(inner_cheese.pasteurised),
    cheesetype: Option::Some(inner_cheese.cheesetype),
    rind: Option::Some(inner_cheese.rind),
    additive: inner_cheese.additive,
    region: inner_cheese.region,
    country: Option::Some(inner_cheese.country),
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

#[post("/update_cheese/<id>", format = "application/json", data = "<updatedCheese>")]
pub fn update_cheese(id: i32, updatedCheese: Json<NewCheese>, conn: CheesesDbConn) -> String {
    let inner_cheese = updatedCheese.into_inner();

    let cheese = cheeses::table.find(id);

    diesel::update(cheese).set(&inner_cheese).execute(&*conn).unwrap();

    // TODO: add wiki photo stuffc
    // TODO: unwrap better result of get_result
    String::from("ok")
}


#[get("/cheeses?<filters..>")]
pub fn get_cheeses(filters: Option<Form<GetFilters>>, conn: CheesesDbConn) -> Json<Vec<Cheese>> {

    let mut query = cheeses::table.into_boxed();


    if let Some(filters) = filters {
        if let Some(pasteurised) = filters.pasteurised {
            query = query.filter(cheeses::pasteurised.eq(pasteurised));
        } 
        if let Some(milk) = filters.milk.clone() {
            query = query.filter(cheeses::milk.eq(milk));
        } 
        if let Some(cheesetype) = filters.cheesetype.clone() {
            query = query.filter(cheeses::cheesetype.eq(cheesetype));
        } 
        if let Some(rind) = filters.rind.clone() {
            query = query.filter(cheeses::rind.eq(rind));
        } 
        if let Some(country) = filters.country.clone() {
            query = query.filter(cheeses::country.eq(country));
        } 

    }
    
    
    let result =  query.load::<db::models::Cheese>(&*conn)
    .expect("Error loading cheeses");
    Json(result)
}

#[get("/cheese/<id>")]
pub fn get_by_id(id: i32, conn: CheesesDbConn) -> Json<Cheese> {
    
    let result = cheeses::table.find(id).first::<db::models::Cheese>(&*conn).expect("Error loading cheese");

    Json(result)
}

#[get("/delete_cheese/<id>")]
pub fn delete(id: i32, conn: CheesesDbConn) -> Result<String, std::io::Error> {
    
    let result = diesel::delete(cheeses::table.find(id)).execute(&*conn);

    match result {
        Ok(x) =>  Result::Ok(String::from("deleted")),
        Err(x) => Result::Err(x.),
    }

}


