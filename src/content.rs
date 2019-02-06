extern crate cursive;
extern crate regex;
extern crate reqwest;
extern crate serde_json;
extern crate url;

use self::regex::Regex;
use cursive::theme::Effect;
use cursive::utils::markup::StyledString;
use cursive::views::Dialog;
use cursive::Cursive;
use reqwest::Url;
use serde_json::Value;
use CONFIGURATION;

use content::url::percent_encoding::{ utf8_percent_encode, DEFAULT_ENCODE_SET };

use std::fs::File;
use std::io::prelude::*;

pub fn query_url_gen(title: &str) -> Url {
    let url = Url::parse_with_params(
        &(CONFIGURATION.wiki_url.clone() + "/w/api.php"),
        &[
            ("action", "query"),
            ("format", "json"),
            ("prop", "extracts|links"),
            ("indexpageids", "1"),
            ("titles", &utf8_percent_encode(&title.replace(" ", "_"), DEFAULT_ENCODE_SET).to_string()[..]),
            ("redirects", "1"),
            ("pllimit", "100"),
            ("explaintext", "1"),
        ],
    ).unwrap();

    let mut f = File::open("~/.taizen_logs").unwrap();
    f.write_all(url.as_str().as_bytes()).unwrap();

    return url;
}

pub fn search_url_gen(search: &str) -> Url {
    let url = Url::parse_with_params(
        &(CONFIGURATION.wiki_url.clone() + "/w/api.php"),
        &[
            ("action", "opensearch"),
            ("format", "json"),
            ("search", &utf8_percent_encode(&search, DEFAULT_ENCODE_SET).to_string()[..]),
            ("limit", "20"),
        ],
    ).unwrap();

    let mut f = File::create("taizen_logs.txt").unwrap();
    f.write_all(url.as_str().as_bytes()).expect("failed to write unicode");
    f.write_all(search.as_bytes()).unwrap();

    return url;
}

pub fn get_extract(v: &Value) -> Result<String, reqwest::Error> {
    let pageid = &v["query"]["pageids"][0];
    let pageid_str = match pageid {
        Value::String(id) => id,
        _ => "-1",
    };

    match &v["query"]["pages"][pageid_str]["extract"] {
        Value::String(extract) => {
            // format to plain text
            let extract = extract.replace("\\\\", "\\");

            Ok(extract.to_string())
        }
        // ignore non strings
        _ => Ok("This page does not exist anymore".to_string()),
    }
}

pub fn extract_formatter(extract: &str) -> StyledString {
    let mut formatted = StyledString::new();

    let heading = Regex::new(r"^== (?P<d>.*) ==$").unwrap();
    let subheading = Regex::new(r"^=== (?P<d>.*) ===$").unwrap();
    let subsubheading = Regex::new(r"^==== (?P<d>.*) ====$").unwrap();

    for line in extract.lines() {
        if heading.is_match(line) {
            formatted.append(StyledString::styled(
                heading.replace(line, "$d"),
                Effect::Bold,
            ));
        } else if subheading.is_match(line) {
            formatted.append(StyledString::styled(
                subheading.replace(line, "$d"),
                Effect::Italic,
            ));
        } else if subsubheading.is_match(line) {
            formatted.append(StyledString::styled(
                subsubheading.replace(line, "$d"),
                Effect::Underline,
            ));
        } else {
            formatted.append(StyledString::plain(line));
        }

        formatted.append(StyledString::plain("\n"))
    }

    formatted
}

pub fn get_search_results(search: &str) -> Result<Vec<String>, reqwest::Error> {
    let url = search_url_gen(search);
    let mut res = reqwest::get(&url[..])?;
    let v: Value = serde_json::from_str(&res.text()?).unwrap_or_else(|e| {
        panic!("Recieved error {:?}", e);
    });

    let mut results: Vec<String> = vec![];
    for item in v[1].as_array().unwrap() {
        if let Value::String(x) = item {
            results.push(x.to_string())
        };
    }

    // let mut f = File::open("taizen_logs.txt").unwrap();
    // for result in &results {
    //     f.write_all(result.as_bytes()).unwrap();
    // }

    Ok(results)
}

pub fn get_links(v: &Value) -> Result<Vec<String>, reqwest::Error> {
    let pageid = &v["query"]["pageids"][0];
    let pageid_str = match pageid {
        Value::String(id) => id,
        _ => panic!("wut"),
    };

    let mut links = vec![];
    match &v["query"]["pages"][pageid_str]["links"] {
        Value::Array(arr) => {
            for item in arr {
                match item["title"] {
                    Value::String(ref title) => links.push(title.to_string()),
                    _ => links.push(String::from("lol")),
                }
            }
        }
        _ => links.push(String::from("lol")),
    };

    Ok(links)
}

pub fn pop_error(s: &mut Cursive, msg: &str) {
    s.add_layer(Dialog::text(msg.to_string()).button("Ok", |s| s.quit()));
}

pub fn handler(e: &reqwest::Error) -> String {
    let mut msg: String = String::new();
    if e.is_http() {
        match e.url() {
            None => msg.push_str(&"No URL given"),
            Some(url) => msg.push_str(&format!("Problem making request to: {}", url)),
        }
    }

    if e.is_redirect() {
        msg.push_str(&"server redirecting too many times or making loop");
    }

    msg
}
