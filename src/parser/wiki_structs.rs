use std::collections::HashMap;
use serde_json::{from_str, Value, Number, Map};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub pageid: Number,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageQuery {
    pub search: Vec<Page>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageListResponse {
    pub query: PageQuery
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PageImage {
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PageImageObject {
    pub images: Vec<PageImage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageImageQuery {
    pub pages: HashMap<String, PageImageObject>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageImageListResponse {
    pub query: PageImageQuery
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageObject {
    pub imageinfo: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageQuery {
    pub pages: HashMap<String, ImageObject>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageListResponse {
    pub query: ImageQuery
}