extern crate reqwest;
extern crate serde_json;
extern crate cursive;

use serde_json::Value;

use cursive::Cursive;
use cursive::{TextView, Dialog, EditView, SelectView};

pub mod content;

fn main() {
    // Initial setup
    let mut main = Cursive::default();

    main.add_layer(TextView::new("Welcome!")));
    main.add_global_callback('q', |s| s.quit());
    main.add_global_callback('s', search()));
}

fn search(s: &mut Cursive){
    fn go(s: &mut Cursive, search: &str) {
        s.pop_layer();
        let search_results: Vec<String> = content::get_search_results();
        let sv = SelectView::with_all_strs(search_results.iter());
    }

    s.add_layer(Dialog::around(EditView::new()
                               .on_submit(render_page())
                               .with_id("search")
                               .fixed_width(10))
                .title("Search for a page")
                .button("Go", |s| {
                    let search_txt = s.call_on_id( "search", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    go(s, search_txt);
                })
                .button("Cancel", |s| s.pop_layer()));
}
