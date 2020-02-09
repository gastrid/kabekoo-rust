
extern crate dotenv;
extern crate diesel_derive_enum;

extern crate r2d2;
extern crate r2d2_diesel;


use diesel::prelude::*;

pub mod models;
pub mod schema;
pub mod conn;
