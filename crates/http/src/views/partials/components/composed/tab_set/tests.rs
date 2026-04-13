use super::*;

#[test]
fn component_renders_local_tabs_root_and_active_tab() {
    let markup = Component {
        id: "tab-set",
        class: "extra-root-class",
        variant: Variant::Standard,
        active_tab_id: Text::from("alpha'\"beta"),
        tabs: tab::Set {
            aria_label: Text::from("Example tabs"),
            style: tab::Style::Standard,
            tabs: tab::List { children: vec![] },
        },
        panes: pane::List { children: vec![] },
    }
    .render()
    .into_string();

    assert!(markup.contains("data-local-tabs-root"));
    assert!(markup.contains("data-local-tabs-active=\"alpha'&quot;beta\""));
    assert!(markup.contains("class=\"tab-set-showcase extra-root-class\""));
}

#[test]
fn from_content_defaults_to_first_tab() {
    let content = content::TabSet {
        tabs: vec![content::Tab {
            id: Text::from("policy"),
            label: content::Label {
                primary: Text::from("Policy"),
                secondary: None,
            },
            icon: None,
            preview: Some(content::Preview {
                code_examples: vec![content::CodeExample {
                    label: Some(Text::from("Example")),
                    code: Text::from("fn main() {}"),
                }],
                image: None,
                badge: None,
            }),
            body: None,
            action: None,
        }],
    };

    let component = Component::from_content(
        ContentProps::builder()
            .id("tab-set")
            .class("extra-root-class")
            .aria_label(Text::from("Solutions"))
            .content(&content)
            .build(),
    );

    assert_eq!(component.active_tab_id, Text::from("policy"));
    assert_eq!(component.tabs.tabs.children.len(), 1);
    assert_eq!(component.panes.children.len(), 1);
}
