extern crate reqwest;
extern crate serde_json;
extern crate cursive;

use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{ TextView, Dialog, EditView, 
    SelectView, OnEventView };
use cursive::theme::PaletteColor::*;
use cursive::theme::Color::*;
use cursive::theme::BaseColor::*;
use cursive::theme;
use cursive::theme::BorderStyle;

pub mod content;
use content::*;

fn main() {
    // Initial setup
    let mut main = Cursive::default();

    // basic theme
    let mut wikitheme = main.current_theme().clone();

    // set the theme's
    // shadow
    wikitheme.shadow = false;
    // border
    wikitheme.borders = BorderStyle::Simple;
    // and palette
    let mut palette: theme::Palette = theme::Palette::default();
    palette.set_color("background"         , Dark(Black));
    palette.set_color("shadow"             , Dark(White));
    palette.set_color("view"               , Dark(Black));
    palette.set_color("primary"            , Dark(White));
    palette.set_color("secondary"          , Dark(Blue));
    palette.set_color("teritary"           , Dark(Green));
    palette.set_color("title_primary"      , Dark(Blue));
    palette.set_color("title_secondary"    , Dark(Green));
    palette.set_color("highlight"          , Dark(Blue));
    palette.set_color("highlight_inactive" , Dark(Red));

    wikitheme.palette = palette;

    // set theme
    main.set_theme(wikitheme);

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
            .on_submit(on_submit);
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
                }));
}

fn on_submit(s: &mut Cursive, name: &String) {
    s.pop_layer();

    let heading: String = name.clone();
    let url = query_url_gen(&name.replace(" ", "_"));
    let res = reqwest::get(&url).unwrap();
    let mut extract = String::new();

    match get_extract(res) {
        Ok(x) => extract = x,
        Err(e) => pop_error(s, handler(e))
    };

    s.add_layer(
        Dialog::around(
            OnEventView::new(TextView::new(extract_formatter(extract)))
            .on_event('t', |s| match s.pop_layer() { _ => () })
            )
        .title(heading)
        .padding_right(2)
        .padding_left(2)
        );
}
