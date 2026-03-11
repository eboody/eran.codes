use maud::Render;
use serde::{Deserialize, Deserializer};

use crate::types::Text;
use crate::views::proper_theme::ThemeColor;

pub(crate) type IconToken = Text;
const FALLBACK_ICON_TOKEN: &str = "circle";
const STYLES: &str = r#"
.ui-icon {
  display: var(--control-inline-display);
  align-items: var(--control-inline-align-items);
  font-size: var(--control-icon-size);
  color: var(--icon-color, currentColor);
}
"#;

pub(crate) fn head_styles() -> maud::Markup {
    crate::views::scoped::style(STYLES)
}

#[derive(Clone, Debug)]
pub(crate) struct Icon {
    pub token: IconToken,
    pub color: Option<ThemeColor>,
}

impl Icon {
    pub(crate) fn from_token(token: impl Into<Text>) -> Self {
        Self {
            token: normalize_icon_token(token.into()),
            color: None,
        }
    }

    pub(crate) fn with_color(mut self, color: ThemeColor) -> Self {
        self.color = Some(color);
        self
    }
}

impl<'de> Deserialize<'de> for Icon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RawIcon {
            Token(Text),
            Object { key: Text },
        }

        let token = match RawIcon::deserialize(deserializer)? {
            RawIcon::Token(token) => token,
            RawIcon::Object { key } => key,
        };

        Ok(Self::from_token(token))
    }
}

impl Render for Icon {
    fn render(&self) -> maud::Markup {
        let icon_class = format!("iconoir-{}", self.token);

        maud::html! {
            @if let Some(color) = &self.color {
                span class="ui-icon" aria-hidden="true" style={ "--icon-color: " (color) ";" } {
                    i class=(icon_class) {}
                }
            } @else {
                span class="ui-icon" aria-hidden="true" {
                    i class=(icon_class) {}
                }
            }
        }
    }
}

fn normalize_icon_token(token: Text) -> IconToken {
    let raw = token.to_string();
    if is_valid_icon_token(&raw) {
        token
    } else {
        Text::from(FALLBACK_ICON_TOKEN)
    }
}

fn is_valid_icon_token(token: &str) -> bool {
    !token.is_empty()
        && token
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}
