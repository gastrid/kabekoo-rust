extern crate kabekoo;
use kabekoo::db::models::Cheese;
use kabekoo::parser::wiki_image::get_wiki_image;

fn main() {

    let result = get_wiki_image("gorgonzola");

    match result {
        Ok(x) => println!("OK: {}", x),
        Err(x) => println!("Err: {}", x),
    }
}
