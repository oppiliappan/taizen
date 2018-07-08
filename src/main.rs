extern crate reqwest;
extern crate serde_json;
extern crate cursive;

use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{ TextView, Dialog, EditView,
    SelectView, OnEventView, LinearLayout };

pub mod content;
use content::*;

pub mod theme;
use theme::*;

fn main() {
    // Initial setup
    let mut main = Cursive::default();

    // set theme
    main.set_theme(theme_gen());

    main.add_global_callback('q', |s| s.quit());
    main.add_global_callback('s', |s| search(s));

    main.run();
}

fn search(s: &mut Cursive){

    fn go(s: &mut Cursive, search: &str) {
        s.pop_layer();
        let mut result = vec![];
        match get_search_results(search) {
            Ok(x) => result = x,
            Err(e) => pop_error(s,handler(e)),
        };
        let choose_result = SelectView::<String>::new()
            .with_all_str(result)
            .on_submit(|s, name| {
                s.pop_layer();
                on_submit(s, name);
            });
        s.add_layer(Dialog::around(choose_result)
                    .title("Search Results")
                    .button("Cancel", |s| match s.pop_layer() { _ => () })
                    .fixed_size(( 45,10 )));
    }

    s.add_layer(Dialog::around(EditView::new()
                               .on_submit(go)
                               .with_id("search")
                               )
                .title("Search for a page")
                .button("Go", |s| {
                    let search_txt = s.call_on_id( "search", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();
                    go(s, &search_txt);
                })
                .button("Cancel", |s| match s.pop_layer(){
                    _ => ()
                })
                .fixed_size(( 35, 5 )));
}

fn on_submit(s: &mut Cursive, name: &String) {
    // get article data
    let heading: String = name.clone();
    let url = query_url_gen(&name.replace(" ", "_"));
    let mut res = reqwest::get(&url).unwrap();
    let mut extract = String::new();
    let mut link_vec: Vec<String> = vec![];

    // handle errors if any
    match get_extract(&mut res) {
        Ok(x) => extract = x,
        Err(e) => pop_error(s, handler(e))
    };
    match get_links(&mut res) {
        Ok(x) => link_vec = x,
        Err(e) => pop_error(s, handler(e))
    };

    // get the act together
    let mut article = TextView::new(heading);
    article.append(String::from("\n\n"));
    article.append(extract_formatter(extract));

    let links = SelectView::<String>::new()
        .with_all_str(link_vec)
        .on_submit(on_submit);
    s.add_layer(
        LinearLayout::horizontal()
        .child(
            OnEventView::new(
                article.fixed_width(72)
                )
            .on_event('t', |s| match s.pop_layer() { _ => () })
              )
        .child(links)
        );
}
