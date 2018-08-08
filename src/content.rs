extern crate cursive;
extern crate regex;
extern crate reqwest;
extern crate serde_json;
extern crate urlencoding;

use self::regex::Regex;
use cursive::theme::Effect;
use cursive::utils::markup::StyledString;
use cursive::views::Dialog;
use cursive::Cursive;
use serde_json::Value;
use CONFIGURATION;

pub fn query_url_gen(title: &str) -> String {
    // query config
    let mut url = CONFIGURATION.wiki_url.clone();
    url.push_str("/w/api.php?");
    url.push_str("action=query&");
    url.push_str("format=json&");
    url.push_str("prop=extracts%7Clinks&");
    url.push_str("indexpageids=1&");
    url.push_str("titles=");
    url.push_str(&urlencoding::encode(&title.replace(" ", "_")));
    url.push_str("&");
    url.push_str("redirects=1&");
    url.push_str("pllimit=100&");
    url.push_str("explaintext=1");
    url
}

pub fn search_url_gen(search: &str) -> String {
    // search config
    let mut url = CONFIGURATION.wiki_url.clone();
    url.push_str("/w/api.php?");
    url.push_str("action=opensearch&");
    url.push_str("format=json&");
    url.push_str("search=");
    url.push_str(&urlencoding::encode(&search));
    url.push_str("&");
    url.push_str("limit=20");
    url
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
        }
    }
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
