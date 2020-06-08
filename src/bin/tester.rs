extern crate kabekoo;
use kabekoo::db::models::Cheese;
use kabekoo::parser::wiki_image::get_wiki_image;

fn main() {

    get_wiki_image("gorgonzola");
}
