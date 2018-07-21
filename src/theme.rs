use cursive::theme;
use cursive::theme::BaseColor::*;
use cursive::theme::BorderStyle;
use cursive::theme::Color::*;
use cursive::theme::PaletteColor::*;

pub fn palette_gen() -> theme::Palette {
    let mut palette = theme::Palette::default();

    palette[Background] = Dark(Black);
    palette[Shadow] = Light(Black);
    palette[View] = Dark(Black);
    palette[Primary] = Dark(White);
    palette[Secondary] = Light(Black);
    palette[Tertiary] = Dark(Green);
    palette[TitlePrimary] = Dark(Blue);
    palette[TitleSecondary] = Dark(Green);
    palette[Highlight] = Dark(Blue);
    palette[HighlightInactive] = Light(Black);

    palette
}

pub fn theme_gen() -> theme::Theme {
    let mut wikitheme = theme::Theme::default();

    wikitheme.shadow = false;
    wikitheme.borders = BorderStyle::Simple;
    wikitheme.palette = palette_gen();

    wikitheme
}
