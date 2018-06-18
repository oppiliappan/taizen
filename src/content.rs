pub fn url_gen(title: &str) -> String {

    title.replace(" ", "%20");

    // query config
    let url = "https://en.wikipedia.org";
    url.push_str("w/api.php?");
    url.push_str("action=query&");
    url.push_str("format=json&");
    url.push_str("prop=extracts&");
    url.push_str(format!("titles={}", title));
    url.push_str("explaintext=1");
}

pub fn get_extract(title: &str, red: Response) -> String {
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

pub fn get_title(title: &str, res: Response) -> String {
    let mut v: Value = serde_json::from_str(&res.text().unwrap()).unwrap_or_else( |e| {
        panic!("Recieved invalid json");
    } );
    format!("{}", &v["query"]["normalized"][0]["to"])
}
