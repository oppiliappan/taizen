extern crate reqwest;
extern crate serde_json;

use serde_json::Value;

pub mod content;

fn main() {
    let url = format!("https://en.wikipedia.org/w/api.php?action=query&format=json&prop=extracts&list=&meta=&indexpageids=1&continue=%7C%7Cimageinfo&titles={}&exlimit=20&explaintext=1&exsectionformat=plain", title);
    let res = reqwest::get(&url);

    match res {
        Ok(res) => {
            if res.status().is_success() {
                content::get_extract("")
            }
        }

        Err(_) {
            panic!("Oh no!");
        }
    }
}
