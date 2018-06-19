extern crate reqwest;
extern crate serde_json;
extern crate cursive;

use serde_json::Value;
use reqwest::Response;

pub fn query_url_gen(title: &str) -> String {

    title.replace(" ", "%20");

    // query config
    let mut url = String::from("https://en.wikipedia.org");
    url.push_str("w/api.php?");
    url.push_str("action=query&");
    url.push_str("format=json&");
    url.push_str("prop=extracts&");
    url.push_str("titles=");
    url.push_str(title);
    url.push_str("&");
    url.push_str("explaintext=1");

    url
}

pub fn search_url_gen(search: &str) -> String {
    // /w/api.php?action=opensearch&format=json&search=dota%202&limit=5;

    search.replace(" ", "%20");

    let mut url = String::from("https://en.wikipedia.org");
    url.push_str("w/api.php?");
    url.push_str("action=opensearch&");
    url.push_str("format=json&");
    url.push_str("search=");
    url.push_str(search);
    url.push_str("&");
    url.push_str("limit=5");

    url

}

pub fn get_extract(title: &str, mut res: reqwest::Response) -> String {
    let mut v: Value = serde_json::from_str(&res.text().unwrap()).unwrap();

    // Fetch page and pageids of requested title(s)
    let pageid = &v["query"]["pageids"][0];
    let pageid_str = match pageid {
        Value::String(id) => id,
        _ => panic!("wut"),
    };

    if pageid_str == "-1" {
        String::from("No such page")
    } else {
        format!("{}", &v["query"]["pages"][pageid_str]["extract"])
    }
}

pub fn get_title(title: &str, mut res: Response) -> String {
    let mut v: Value = serde_json::from_str(&res.text().unwrap())
        .unwrap_or_else( |e| {
            panic!("Recieved error {:?}", e);
        } );
    format!("{}", &v["query"]["normalized"][0]["to"])
}

pub fn get_search_results(search: &str) -> Vec<String> {

    let url = search_url_gen(search);
    let res = reqwest::get(&url[..]);

    match res {
        Ok(mut res) => {
            if res.status().is_success() {
                let mut v: Value = serde_json::from_str(&res.text().unwrap())
                    .unwrap_or_else( |e| {
                        panic!("Recieved error {:?}", e);
                    } );

                let mut results: Vec<String> = vec![];
                for item in v[1].as_array().unwrap() {
                    match item {
                        Value::String(x) => results.push(x.to_string()),
                        _ => (),
                    }
                }
                results
            } else {
                panic!("Encountered Error {}", res.status());
            }
        },
        _ => {
            panic!("Unable to parse url");
        }
    }

}
