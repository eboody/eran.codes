use std::fmt::Write;

use bon::Builder;
use maud::{Escaper, Render};

use crate::types::Text;

use super::{DataAttr, Role, Variant};

#[derive(Clone, Debug, Builder)]
pub struct Button {
    pub label: Text,
    #[builder(default)]
    pub variant: Variant,
    #[builder(default)]
    pub role: Role,
    #[builder(default)]
    pub data_attrs: Vec<DataAttr>,
}

impl Render for Button {
    fn render_to(&self, buffer: &mut String) {
        let class_name = self.variant.class_name();

        match &self.role {
            Role::Button => {
                buffer.push_str("<button");
                write_attr(buffer, "type", "button");
                write_attr(buffer, "class", class_name);
                write_flag_attr(buffer, "data-button");
                write_data_attrs(buffer, &self.data_attrs);
                buffer.push('>');
                write_text(buffer, &self.label);
                buffer.push_str("</button>");
            }
            Role::Submit { name, value } => {
                buffer.push_str("<button");
                write_attr(buffer, "type", "submit");
                write_attr(buffer, "class", class_name);
                write_flag_attr(buffer, "data-button");
                if let Some(name) = name {
                    write_attr(buffer, "name", name);
                }
                if let Some(value) = value {
                    write_attr(buffer, "value", value);
                }
                write_data_attrs(buffer, &self.data_attrs);
                buffer.push('>');
                write_text(buffer, &self.label);
                buffer.push_str("</button>");
            }
            Role::Link { href, external } => {
                buffer.push_str("<a");
                write_attr(buffer, "class", class_name);
                write_flag_attr(buffer, "data-button");
                write_attr(buffer, "href", href);
                if *external {
                    write_attr(buffer, "target", "_blank");
                    write_attr(buffer, "rel", "noopener noreferrer");
                }
                write_data_attrs(buffer, &self.data_attrs);
                buffer.push('>');
                write_text(buffer, &self.label);
                buffer.push_str("</a>");
            }
        }
    }
}

fn write_attr(buffer: &mut String, name: &str, value: impl std::fmt::Display) {
    buffer.push(' ');
    buffer.push_str(name);
    buffer.push_str("=\"");
    write_text(buffer, value);
    buffer.push('"');
}

fn write_flag_attr(buffer: &mut String, name: &str) {
    buffer.push(' ');
    buffer.push_str(name);
}

fn write_data_attrs(buffer: &mut String, data_attrs: &[DataAttr]) {
    for data_attr in data_attrs {
        match data_attr {
            DataAttr::Flag(name) => {
                write_flag_attr(buffer, &name.to_string());
            }
            DataAttr::Value { name, value } => {
                buffer.push(' ');
                buffer.push_str(&name.to_string());
                buffer.push_str("=\"");
                let _ = write!(Escaper::new(buffer), "{value}");
                buffer.push('"');
            }
        }
    }
}

fn write_text(buffer: &mut String, value: impl std::fmt::Display) {
    let _ = write!(Escaper::new(buffer), "{value}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_primary_button_by_default() {
        let markup = Button::builder()
            .label(Text::from("Run"))
            .build()
            .render()
            .into_string();

        assert_eq!(
            markup,
            "<button type=\"button\" class=\"button\" data-button>Run</button>"
        );
    }

    #[test]
    fn renders_secondary_submit_with_name_and_value() {
        let markup = Button::builder()
            .label(Text::from("Approve"))
            .variant(Variant::Secondary)
            .role(Role::submit_with("decision", "approve"))
            .build()
            .render()
            .into_string();

        assert!(markup.contains("type=\"submit\""));
        assert!(markup.contains("class=\"button secondary\""));
        assert!(markup.contains("data-button"));
        assert!(markup.contains("name=\"decision\""));
        assert!(markup.contains("value=\"approve\""));
    }

    #[test]
    fn renders_external_link_variant() {
        let markup = Button::builder()
            .label(Text::from("Docs"))
            .variant(Variant::Secondary)
            .role(Role::external_link("https://example.com"))
            .build()
            .render()
            .into_string();

        assert!(markup.starts_with("<a "));
        assert!(markup.contains("href=\"https://example.com\""));
        assert!(markup.contains("target=\"_blank\""));
        assert!(markup.contains("rel=\"noopener noreferrer\""));
        assert!(markup.contains("data-button"));
        assert!(!markup.contains("type=\"submit\""));
    }

    #[test]
    fn renders_boolean_and_valued_data_attrs() {
        let markup = Button::builder()
            .label(Text::from("Clear"))
            .variant(Variant::Secondary)
            .data_attrs(vec![
                DataAttr::flag("data-op-filter-clear"),
                DataAttr::value("data-on:click", "$operations_filter_query = ''"),
            ])
            .build()
            .render()
            .into_string();

        assert!(markup.contains("class=\"button secondary\""));
        assert!(markup.contains("data-op-filter-clear"));
        assert!(markup.contains("data-on:click=\"$operations_filter_query = ''\""));
    }
}
