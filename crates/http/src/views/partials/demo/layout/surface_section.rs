use std::fmt::Write;

use bon::Builder;
use maud::{Escaper, Markup, Render};

use crate::types::Text;

use super::SectionHeader;
use crate::views::partials::components::SectionHeaderDensity;

#[derive(Clone, Debug)]
pub enum SurfaceSectionAttr {
    Flag(Text),
    Value { name: Text, value: Text },
}

impl SurfaceSectionAttr {
    pub fn flag(name: impl Into<Text>) -> Self {
        let name = name.into();
        debug_assert!(name.to_string().starts_with("data-"));
        Self::Flag(name)
    }

    pub fn value(name: impl Into<Text>, value: impl Into<Text>) -> Self {
        let name = name.into();
        debug_assert!(name.to_string().starts_with("data-"));
        Self::Value {
            name,
            value: value.into(),
        }
    }
}

#[derive(Debug, Builder)]
pub struct SurfaceSection {
    pub title: Text,
    pub subtitle: Option<Text>,
    pub action: Option<crate::views::partials::button::Button>,
    pub content: Markup,
    pub id: Option<Text>,
    pub extra_class: Option<Text>,
    #[builder(default)]
    pub attrs: Vec<SurfaceSectionAttr>,
    #[builder(default)]
    pub header_density: SectionHeaderDensity,
}

impl Render for SurfaceSection {
    fn render_to(&self, buffer: &mut String) {
        buffer.push_str("<section");
        if let Some(id) = &self.id {
            write_attr(buffer, "id", id);
        }
        write_attr(buffer, "class", self.class_attr());
        write_attrs(buffer, &self.attrs);
        buffer.push('>');
        SectionHeader::builder()
            .title(self.title.clone())
            .maybe_subtitle(self.subtitle.clone())
            .maybe_action(self.action.clone())
            .density(self.header_density)
            .build()
            .render_to(buffer);
        self.content.render_to(buffer);
        buffer.push_str("</section>");
    }
}

impl SurfaceSection {
    fn class_attr(&self) -> String {
        match &self.extra_class {
            Some(extra_class) => format!("u-surface-card {extra_class}"),
            None => "u-surface-card".to_string(),
        }
    }
}

fn write_attrs(buffer: &mut String, attrs: &[SurfaceSectionAttr]) {
    for attr in attrs {
        match attr {
            SurfaceSectionAttr::Flag(name) => write_flag_attr(buffer, name),
            SurfaceSectionAttr::Value { name, value } => write_attr(buffer, name, value),
        }
    }
}

fn write_attr(buffer: &mut String, name: impl std::fmt::Display, value: impl std::fmt::Display) {
    buffer.push(' ');
    let _ = write!(buffer, "{name}");
    buffer.push_str("=\"");
    write_text(buffer, value);
    buffer.push('"');
}

fn write_flag_attr(buffer: &mut String, name: impl std::fmt::Display) {
    buffer.push(' ');
    let _ = write!(buffer, "{name}");
}

fn write_text(buffer: &mut String, value: impl std::fmt::Display) {
    let _ = write!(Escaper::new(buffer), "{value}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_root_id_class_and_data_attrs() {
        let markup = SurfaceSection::builder()
            .id(Text::from("example-section"))
            .extra_class(Text::from("example-extra"))
            .title(Text::from("Example"))
            .subtitle(Text::from("Subtitle"))
            .attrs(vec![
                SurfaceSectionAttr::flag("data-example"),
                SurfaceSectionAttr::value("data-state", "ready"),
            ])
            .header_density(SectionHeaderDensity::Compact)
            .content(maud::html! { p { "content" } })
            .build()
            .render()
            .into_string();

        assert!(markup.contains("id=\"example-section\""));
        assert!(markup.contains("class=\"u-surface-card example-extra\""));
        assert!(markup.contains("data-example"));
        assert!(markup.contains("data-state=\"ready\""));
        assert!(markup.contains("<h2>Example</h2>"));
        assert!(markup.contains("u-section-header--compact"));
    }
}
