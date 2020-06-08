extern crate reqwest;
extern crate json;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use serde_json::{from_str, Value, Number};
use serde::{Deserialize, Serialize};

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

use reqwest::blocking;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ');



#[derive(Serialize, Deserialize, Debug)]
struct Page {
    pageid: Number,
}

#[derive(Serialize, Deserialize, Debug)]
struct Query {
    search: Vec<Page>
}

#[derive(Serialize, Deserialize, Debug)]
struct PageListResponse {
    query: Query
}


pub fn get_wiki_image(name: &str) -> Result<&str, Error> {
    // TODO: removing trailing spaces
    let encoded_name = utf8_percent_encode(name, FRAGMENT).to_string();
    let clean_name = encoded_name.trim();

    // Step 1: get the pageId
    // TODO: error handling
    let page_id_url = format!("https://en.wikipedia.org/w/api.php?action=query&list=search&format=json&srsearch={}%20cheese", clean_name);
    let page_list_response = reqwest::blocking::get(&page_id_url).unwrap().json::<PageListResponse>().unwrap();

    let page_id = &page_list_response.query.search[0].pageid;

    println!("{}", page_id);

    // Step 2: get the image title
    let image_title_url = format!("https://en.wikipedia.org/w/api.php?action=query&pageids={}&prop=images&format=json", page_id);
    let image_list_response = reqwest::blocking::get(&page_id_url).unwrap().json::<Value>().unwrap();

    
    
    println!("{:?}", image_list_response);
    Result::Ok("ok")
}