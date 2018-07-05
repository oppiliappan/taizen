use cursive::theme::Color::*;
use cursive::theme::BaseColor::*;
use cursive::theme::BorderStyle;
use cursive::theme;

pub fn palette_gen() -> theme::Palette {
    let mut palette: theme::Palette = theme::Palette::default();

    palette.set_color("background"         , Dark(Black));
    palette.set_color("shadow"             , Dark(White));
    palette.set_color("view"               , Dark(Black));
    palette.set_color("primary"            , Dark(White));
    palette.set_color("secondary"          , Light(Black));
    palette.set_color("teritary"           , Dark(Green));
    palette.set_color("title_primary"      , Dark(Blue));
    palette.set_color("title_secondary"    , Dark(Green));
    palette.set_color("highlight"          , Dark(Blue));
    palette.set_color("highlight_inactive" , Light(Black));

    palette
}

pub fn theme_gen() -> theme::Theme {
    let mut wikitheme = theme::load_default();

    wikitheme.shadow = false;
    wikitheme.borders = BorderStyle::Simple;
    wikitheme.palette = palette_gen();

    wikitheme
}
