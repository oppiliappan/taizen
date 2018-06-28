extern crate reqwest;
extern crate serde_json;
extern crate cursive;

use cursive::Cursive;
use cursive::views::Dialog;
use serde_json::Value;
use reqwest::Response;

pub fn query_url_gen(title: &str) -> String {
    // query config
    let mut url = String::from("https://en.wikipedia.org");
    url.push_str("/w/api.php?");
    url.push_str("action=query&");
    url.push_str("format=json&");
    url.push_str("prop=extracts&");
    url.push_str("indexpageids=1&");
    url.push_str("titles=");
    url.push_str(title);
    url.push_str("&");
    url.push_str("explaintext=1");
    url
}

pub fn search_url_gen(search: &str) -> String {
    // search config
    search.replace(" ", "%20");
    let mut url = String::from("https://en.wikipedia.org");
    url.push_str("/w/api.php?");
    url.push_str("action=opensearch&");
    url.push_str("format=json&");
    url.push_str("search=");
    url.push_str(search);
    url.push_str("&");
    url.push_str("limit=20");

    url

}

pub fn get_extract(mut res: Response) -> Result<String, reqwest::Error> {
    let v: Value = match serde_json::from_str(&res.text()?) {
        Ok(x) => x,
        Err(x) => panic!("Failed to parse json\nReceived error {}", x),
    };
    let pageid = &v["query"]["pageids"][0];
    let pageid_str = match pageid {
        Value::String(id) => id,
        _ => panic!("wut"),
    };

    match &v["query"]["pages"][pageid_str]["extract"] {
        Value::String(extract) => {
            // format to plain text
            extract.replace("\\\\", "\\");

            Ok(format!("{}", extract))
        }
        // ignore non strings
        _ => Ok(format!(""))
    }
}

pub fn get_search_results(search: &str) -> Result<Vec<String>, reqwest::Error> {
    let url = search_url_gen(search);
    let mut res = reqwest::get(&url[..])?;
    let v: Value = serde_json::from_str(&res.text().unwrap())
        .unwrap_or_else( |e| {
            panic!("Recieved error {:?}", e);
        } );

    let mut results: Vec<String> = vec![];
    for item in v[1].as_array().unwrap() {
        match item {
            Value::String(x) => results.push(x.to_string()),
            // ignore non strings
            _ => (),
        }
    }
    Ok(results)
}

pub fn pop_error(s: &mut Cursive, msg: String) {
    s.add_layer(Dialog::text(format!("{}", msg))
                .button("Ok", |s| s.quit()));
}

pub fn handler(e: reqwest::Error) -> String {
    let mut msg: String = String::new();
    if e.is_http() {
        match e.url() {
            None => msg.push_str(&format!("No URL given")),
            Some(url) => msg.push_str(&format!("Problem making request to: {}", url)),
        }
    }

    if e.is_redirect() {
        msg.push_str(&format!("server redirecting too many times or making loop"));
    }

    msg
}
