extern crate clap;
extern crate cursive;
extern crate reqwest;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

use cursive::views::{
    Dialog, DummyView, EditView, LinearLayout, OnEventView, SelectView, TextView,
};
use cursive::Cursive;
use cursive::{traits::*, CursiveExt};
use serde_json::Value;

use clap::Parser;

pub mod program_arguments;
use program_arguments::ProgramArguments;

pub mod content;
use content::*;

pub mod theme;
use theme::*;

struct Configuration {
    wiki_url: String,
}

lazy_static! {
    static ref CONFIGURATION: Configuration = parse_arguments();
}

fn parse_arguments() -> Configuration {
    let program_args = ProgramArguments::parse();

    let lang = program_args.lang.unwrap_or("en".to_string());
    let wiki_url = program_args
        .url
        .unwrap_or(format!("https://{}.wikipedia.org", lang));

    Configuration { wiki_url }
}

fn main() {
    let _ = parse_arguments();

    // Initial setup
    let mut main = Cursive::default();

    // Set theme
    main.set_theme(theme_gen());

    main.add_global_callback('q', |s| s.quit());
    main.add_global_callback('s', |s| search(s));

    main.add_layer(TextView::new(
        "\t  TAIZEN\r\nHit s to search\r\nHit q to quit\r\nHit t to pop layer",
    ));

    main.run();
}

fn search(s: &mut Cursive) {
    fn go(s: &mut Cursive, search: &str) {
        s.pop_layer();
        let mut result = vec![];
        match get_search_results(&search) {
            Ok(x) => result = x,
            Err(e) => pop_error(s, &handler(&e)),
        };
        let choose_result = SelectView::<String>::new()
            .with_all_str(result)
            .on_submit(|s, name| {
                s.pop_layer();
                on_submit(s, name);
            })
            .scrollable();
        s.add_layer(
            Dialog::around(choose_result)
                .title("Search Results")
                .button("Cancel", |s| match s.pop_layer() {
                    _ => (),
                })
                .fixed_size((45, 10)),
        );
    }

    s.add_layer(
        Dialog::around(EditView::new().on_submit(go).with_name("search"))
            .title("Search for a page")
            .button("Go", |s| {
                let search_txt = s
                    .call_on_name("search", |v: &mut EditView| v.get_content())
                    .unwrap();
                go(s, &search_txt);
            })
            .button("Cancel", |s| match s.pop_layer() {
                _ => (),
            })
            .fixed_size((35, 5)),
    );
}

fn on_submit(s: &mut Cursive, name: &str) {
    // get article data
    let url = query_url_gen(name);
    let mut extract = String::new();
    let mut link_vec: Vec<String> = vec![];

    let mut res = reqwest::get(url).unwrap();
    let v: Value = res.json().expect("Failed to parse json");

    match get_extract(&v) {
        Ok(x) => extract = x,
        Err(e) => pop_error(s, &handler(&e)),
    };
    match get_links(&v) {
        Ok(x) => link_vec = x,
        Err(e) => pop_error(s, &handler(&e)),
    };

    // get the act together
    let article_content = TextView::new(extract_formatter(&extract)).scrollable();

    let links = SelectView::<String>::new()
        .with_all_str(link_vec)
        .on_submit(on_submit)
        .scrollable()
        .fixed_width(20);

    s.add_layer(
        Dialog::around(
            OnEventView::new(
                LinearLayout::horizontal()
                    .child(article_content.fixed_width(72))
                    .child(DummyView)
                    .child(links),
            )
            .on_event('t', |s| match s.pop_layer() {
                _ => (),
            }),
        )
        .title(name),
    );
}
