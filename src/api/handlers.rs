#![feature(proc_macro_hygiene, decl_macro)]

use crate::db;
use diesel::prelude::*;
use crate::db::schema::*;
use db::models::Cheese;
// use rocket::response::status::NotFound;
// use rocket::response::Result;
use rocket::http::Status;
use rocket_contrib::json::Json;




use rocket_contrib::databases::diesel as rockdiesel;

#[database("cheeses")]
pub struct CheesesDbConn(rockdiesel::PgConnection);

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/cheeses")]
pub fn get_cheeses(conn: CheesesDbConn) -> Result<Json<Vec<Cheese>>, Status> {
    use db::schema::cheeses::dsl::{cheeses, pasteurised};
        cheeses.filter(pasteurised.eq(true))
        .limit(5)
        .load::<db::models::Cheese>(&*conn)
        .map(|chss| Json(chss))
        .map_err(|error| Status::NotFound)
    
}

#[get("/make_cheese")]
pub fn make_cheeses(conn: CheesesDbConn) -> String {
    use db::schema::cheeses::dsl::{cheeses, pasteurised};
    let new_cheese = Cheese {
        name: format!("amoozhbu"),
        milk: Milk::Cow,
        pasteurised: true,
        cheesetype: CheeseType::ColdPressed,
        rind: Rind::Natural,
        country: Country::Ireland,
        photo: Option::None,
        additive: Option::None,
        region: Option::None,
        rating: Option::None,
        comment: Option::None,
        maturity: Option::None
    };

    
}

// #[get("/cheeses")]
// pub fn get_cheeses() -> String {
//     use db::schema::cheeses::dsl::{cheeses, pasteurised};
//     let connection = db::conn::establish_connection();
//     let results = cheeses.filter(pasteurised.eq(true))
//         .limit(5)
//         .load::<db::models::Cheese>(&connection)
//         .expect("Error loading posts");

//     println!("Displaying {} posts", results.len());

//     let mut res: String = String::from("");
//     for cheese in results {
//         res.push_str(&cheese.name);
//         res.push_str(&cheese.milk.to_print());
//     }
//     res
// }

