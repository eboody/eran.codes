use super::*;

#[test]
fn defaults_to_h2_heading() {
    let markup = SectionHeader::builder()
        .title(Text::from("Live chat room"))
        .build()
        .render()
        .into_string();

    assert!(markup.contains("<h2>Live chat room</h2>"));
}

#[test]
fn can_render_h1_heading() {
    let markup = SectionHeader::builder()
        .title(Text::from("Chat room"))
        .level(SectionHeaderLevel::H1)
        .action(
            button::Button::builder()
                .label(Text::from("Inspect"))
                .role(button::Role::link("/lab"))
                .build(),
        )
        .build()
        .render()
        .into_string();

    assert!(markup.contains("<h1>Chat room</h1>"));
    assert!(markup.contains("data-section-header-actions"));
}

#[test]
fn can_render_compact_density() {
    let markup = SectionHeader::builder()
        .title(Text::from("Sensitive record proof"))
        .density(Density::Compact)
        .build()
        .render()
        .into_string();

    assert!(markup.contains("class=\"u-section-header u-section-header--compact\""));
}
