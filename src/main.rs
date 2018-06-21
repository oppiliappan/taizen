extern crate reqwest;
extern crate serde_json;
extern crate cursive;

use serde_json::Value;

use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{TextView, Dialog, EditView, SelectView};

pub mod content;
use content::*;

fn main() {
    // Initial setup
    let mut main = Cursive::default();

    main.add_layer(TextView::new("Welcome!"));
    main.add_global_callback('q', |s| s.quit());
    main.add_global_callback('s', |s| search(s));

    main.run();
}

fn search(s: &mut Cursive){

    fn go(s: &mut Cursive, search: &str) {
        s.pop_layer();
        let mut result;
        match get_search_results(search) {
            Ok(x) => result = x,
            Err(e) => pop_error(s,handler(e)),
        };
        let choose_result = SelectView::new().with_all_str(result);
    }

    s.add_layer(Dialog::around(EditView::new()
                               .on_submit(go)
                               .with_id("search")
                               .fixed_width(10))
                .title("Search for a page")
                .button("Go", |s| {
                    let search_txt = s.call_on_id( "search", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();
                    go(s, &search_txt);
                })
                .button("Cancel", |s| match s.pop_layer(){
                    Some(_) => (),
                    None => (),
                }));
}
