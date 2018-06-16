extern crate reqwest;
extern crate serde_json;

use serde_json::Value;

fn main() {
    println!("{}", get_extract("scale"));
}

fn get_extract(title: &str) -> String {
    let url = format!("https://en.wikipedia.org/w/api.php?action=query&format=json&prop=extracts&list=&meta=&indexpageids=1&continue=%7C%7Cimageinfo&titles={}&exlimit=20&explaintext=1&exsectionformat=plain", title);
    let res = reqwest::get(&url);

    match res {
        Ok(mut res) => {
            if res.status().is_success() {
                let mut v: Value = serde_json::from_str(&res.text().unwrap()).unwrap();

                // Fetch page and pageids of requested title(s)
                let pageid = &v["query"]["pageids"][0];
                let pageid_str = match pageid {
                    Value::String(id) => id,
                    _ => panic!("wut"),
                };

                format!("{:#}", &v["query"]["pages"][pageid_str]["extract"])
            } else {
                format!("Error while parsing url.\nRecieved {}", res.status())
            }
        },
        Err(_) => {
            format!("Failed to parse URL")
        }
    }
}
