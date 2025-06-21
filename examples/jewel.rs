#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // No console on release

use fltk::{
    button::Button,
    enums::{Color, Event},
    prelude::*,
    *,
};
use fltk_theme::{color_themes, reset_color_map, ColorMap, ColorTheme, SchemeType, WidgetScheme};

const JEWEL_THEMES: &[&[ColorMap]] = &[
    &color_themes::jewel::DIAMOND_DARK,
    &color_themes::jewel::EMERALD_DARK,
    &color_themes::jewel::AMETHYST_DARK,
    &color_themes::jewel::CITRINE_DARK,
    &color_themes::jewel::RUBY_DARK,
    &color_themes::jewel::PEARL_DARK,
    &color_themes::jewel::DIAMOND_LIGHT,
    &color_themes::jewel::EMERALD_LIGHT,
    &color_themes::jewel::AMETHYST_LIGHT,
    &color_themes::jewel::CITRINE_LIGHT,
    &color_themes::jewel::RUBY_LIGHT,
    &color_themes::jewel::PEARL_LIGHT,
];

fn main() {
    let a = app::App::default();
    WidgetScheme::new(SchemeType::Jewel).apply();
    ColorTheme::new(&color_themes::jewel::DIAMOND_DARK).apply();

    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Jewel Scheme & Colors");
    let mut parent = fltk::group::Group::default_fill();
    parent.set_frame(enums::FrameType::FlatBox);

    let mut col = group::Flex::default()
        .with_size(340, 240)
        .center_of_parent()
        .column();
    col.set_margins(80, 40, 80, 40);
    col.set_frame(enums::FrameType::BorderBox);

    let mut color_choice = menu::Choice::default().with_label("Colors");
    color_choice.add_choice(
        "Diamond Dark|Emerald Dark|Amethyst Dark|Citrine Dark|Ruby Dark|_Pearl Dark|Diamond Light|Emerald Light|Amethyst Light|Citrine Light|Ruby Light|Pearl Light",
    );
    color_choice.set_menu_frame(enums::FrameType::BorderBox);
    color_choice.set_align(enums::Align::TopLeft);
    color_choice.set_value(0);

    let mut input = input::Input::default();
    input.set_frame(enums::FrameType::BorderBox);
    input.set_value("Lorem ipsum dolor sit amet");

    let mut check = button::CheckButton::default().with_label("  Check");
    check.set_value(true);

    let mut round = button::RoundButton::default().with_label("  Round");
    round.set_value(true);

    let mut reset = create_btn("Reset Colors");
    // create_btn("Click!");

    col.end();
    win.end();
    win.make_resizable(true);
    win.show();

    color_choice.set_callback(move |c| {
        reset_color_map();
        ColorTheme::new(JEWEL_THEMES[c.value() as usize]).apply();
    });

    reset.set_callback(move |_| {
        color_choice.set_value(-1);
        reset_color_map();
    });

    a.run().unwrap();
}

fn create_btn(txt: &str) -> Button {
    let mut b = Button::default().with_label(txt);
    b.handle(|b, e| match e {
        Event::Enter => {
            b.set_color(Color::Background2);
            b.redraw();
            true
        }
        Event::Leave => {
            b.set_color(Color::Background);
            b.redraw();
            true
        }
        _ => false,
    });
    b
}
