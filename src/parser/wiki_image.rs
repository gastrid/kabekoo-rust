extern crate reqwest;
extern crate json;
use std::io::{Error, ErrorKind};

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use crate::parser::wiki_structs;

use reqwest::blocking;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ');


pub fn get_wiki_image(name: &str) -> Result<String, Error> {
    let mut result = String::new();

    let encoded_name = utf8_percent_encode(name, FRAGMENT).to_string();
    let clean_name = encoded_name.trim();

    // Step 1: get the pageId
    // TODO: error handling
    let page_url = format!("https://en.wikipedia.org/w/api.php?action=query&list=search&format=json&srsearch={}%20cheese", clean_name);
    let page_list = reqwest::blocking::get(&page_url).unwrap().json::<wiki_structs::PageListResponse>().unwrap();

    let page_id = &page_list.query.search[0].pageid.to_string();


    // Step 2: get the image title
    let images_url = format!("https://en.wikipedia.org/w/api.php?action=query&pageids={}&prop=images&format=json", page_id);
    let images_response = reqwest::blocking::get(&images_url).unwrap().json::<wiki_structs::PageImageListResponse>().unwrap();

    let first_image = &images_response.query.pages[&*page_id].images[0].title;

    // Step 3: get URL
    let encoded_image_name = utf8_percent_encode(first_image, FRAGMENT).to_string();
    let clean_image_name = encoded_image_name.trim();
    

    let image_prop_url = format!("https://www.mediawiki.org/w/api.php?action=query&titles={}&prop=imageinfo&iiprop=url&format=json", clean_image_name);
    let image_response = reqwest::blocking::get(&image_prop_url).unwrap().json::<wiki_structs::ImageListResponse>().unwrap();

    let image_url = &image_response.query.pages["-1"].imageinfo[0].url;
    result.push_str(image_url);
    
    // PICKUP: here, not sure how to deal with this
    Result::Ok(result)
}