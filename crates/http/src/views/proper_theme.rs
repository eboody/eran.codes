use std::sync::LazyLock;

use bon::Builder;
use csscolorparser::Color;
use maud::Render;
use modum::modum;
use nutype::nutype;

#[nutype(derive(Clone, Debug, PartialEq, AsRef, Deref, From, Into))]
pub struct ThemeColor(Color);

impl Render for ThemeColor {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (maud::display(self.as_ref()))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Builder)]
pub struct Palette {
    #[builder(into)]
    pub main: ThemeColor,
    #[builder(into)]
    pub darker: ThemeColor,
    #[builder(into)]
    pub lighter: ThemeColor,
    #[builder(into)]
    pub lightest: ThemeColor,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub teal: Palette,
    pub pink: Palette,
    pub green: Palette,
    pub purple: Palette,
    pub yellow: Palette,
    pub gray: Palette,
    pub red: ThemeColor,
    pub black: ThemeColor,
    pub white: ThemeColor,
    pub transparent: ThemeColor,
}

pub static THEME: LazyLock<Theme> = LazyLock::new(|| Theme {
    teal: Palette::builder()
        .main("#6FDDDB")
        .darker("#2BB4B2")
        .lighter("#7EE1DF")
        .lightest("#B2EDEC")
        .build(),
    pink: Palette::builder()
        .main("#E93EF5")
        .darker("#C70BD4")
        .lighter("#F5A4FA")
        .lightest("#FCE1FD")
        .build(),
    green: Palette::builder()
        .main("#54D072")
        .darker("#30AF4F")
        .lighter("#82DD98")
        .lightest("#B4EAC1")
        .build(),
    purple: Palette::builder()
        .main("#8C18FB")
        .darker("#7204DB")
        .lighter("#B162FC")
        .lightest("#D0A1FD")
        .build(),
    yellow: Palette::builder()
        .main("#E1E862")
        .darker("#BAC31D")
        .lighter("#EFF3AC")
        .lightest("#FAFBE3")
        .build(),
    gray: Palette::builder()
        .main("#4A4A4A")
        .darker("#3D3D3D")
        .lighter("#939393")
        .lightest("#C4C4C4")
        .build(),
    red: "#FF5854".into(),
    black: "#000000".into(),
    white: "#FFFFFF".into(),
    transparent: "transparent".into(),
});

impl From<&str> for ThemeColor {
    fn from(value: &str) -> Self {
        match Color::from_html(value) {
            Ok(color) => Self::from(color),
            Err(error) => panic!("invalid theme color literal `{value}`: {error}"),
        }
    }
}
