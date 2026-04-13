use super::*;


#[test]
fn mobile_nav_wraps_compact_links_without_horizontal_scrolling() {
    let markup = NavBar::builder()
        .brand(
            NavBrand::builder()
                .label(Text::from("eran.codes"))
                .href(Text::from("/"))
                .light_logo_src(Text::from("/static/eran.codes-light.svg"))
                .dark_logo_src(Text::from("/static/eran.codes-dark.svg"))
                .build(),
        )
        .links(
            NavLinkList::builder()
                .children(vec![
                    NavLink::builder()
                        .label(Text::from("Live Proof"))
                        .compact_label(Text::from("Live"))
                        .href(Text::from("/lab"))
                        .build(),
                    NavLink::builder()
                        .label(Text::from("Current Proof"))
                        .compact_label(Text::from("Current"))
                        .href(Text::from("/work/sensitive-sync"))
                        .build(),
                ])
                .build(),
        )
        .meta_links(
            NavLinkList::builder()
                .role(NavLinkListRole::Meta)
                .children(vec![
                    NavLink::builder()
                        .label(Text::from("GitHub"))
                        .compact_label(Text::from("GitHub"))
                        .href(Text::from("https://github.com/eboody"))
                        .external(true)
                        .build(),
                ])
                .build(),
        )
        .auth(NavAuth::Guest(
            NavGuestAuth::builder()
                .sign_in_href(Text::from("/login"))
                .create_account_href(Text::from("/register"))
                .build(),
        ))
        .build()
        .render()
        .into_string();

    assert!(markup.contains("white-space: nowrap;"));
    assert!(markup.contains("flex-wrap: wrap;"));
    assert!(!markup.contains("overscroll-behavior-x: contain;"));
    assert!(markup.contains("grid-template-columns: repeat(2, minmax(0, 1fr));"));
    assert!(markup.contains("[data-nav-link-label='full'] {\n    display: inline;"));
    assert!(markup.contains("[data-nav-link-label='compact'] {\n    display: none;"));
    assert!(markup.contains("grid-template-columns: minmax(0, 1fr) auto;"));
    assert!(markup.contains("data-nav-list=\"meta\""));
    assert!(markup.contains("data-nav-link-label=\"compact\""));
    assert!(markup.contains("data-nav-guest-auth"));
}

#[test]
fn guest_nav_marks_create_account_as_cta() {
    let markup = NavBar::builder()
        .brand(
            NavBrand::builder()
                .label(Text::from("eran.codes"))
                .href(Text::from("/"))
                .light_logo_src(Text::from("/static/eran.codes-light.svg"))
                .dark_logo_src(Text::from("/static/eran.codes-dark.svg"))
                .build(),
        )
        .links(
            NavLinkList::builder()
                .children(vec![NavLink::builder()
                    .label(Text::from("Live Proof"))
                    .href(Text::from("/lab"))
                    .build()])
                .build(),
        )
        .auth(NavAuth::Guest(
            NavGuestAuth::builder()
                .sign_in_href(Text::from("/login"))
                .create_account_href(Text::from("/register"))
                .build(),
        ))
        .build()
        .render()
        .into_string();

    assert!(markup.contains("data-nav-guest-auth"));
    assert!(markup.contains("class=\"button secondary\""));
    assert!(markup.contains("href=\"/register\""));
    assert!(markup.contains(">Create account<"));
    assert!(markup.contains("data-nav-sign-in-action"));
    assert!(markup.contains("data-nav-create-account-action"));
}

#[test]
fn compact_guest_switch_renders_single_nav_link() {
    let markup = NavBar::builder()
        .brand(
            NavBrand::builder()
                .label(Text::from("eran.codes"))
                .href(Text::from("/"))
                .light_logo_src(Text::from("/static/eran.codes-light.svg"))
                .dark_logo_src(Text::from("/static/eran.codes-dark.svg"))
                .build(),
        )
        .links(NavLinkList::builder().children(vec![]).build())
        .auth(NavAuth::Switch(
            NavGuestSwitch::builder()
                .label(Text::from("Create account"))
                .href(Text::from("/register"))
                .build(),
        ))
        .build()
        .render()
        .into_string();

    assert!(markup.contains("data-nav-layout=\"split\""));
    assert!(markup.contains("<a data-nav-link data-nav-auth-switch href=\"/register\">Create account</a>"));
    assert!(!markup.contains("class=\"button\""));
}

#[test]
fn signed_in_nav_exposes_split_auth_status() {
    let markup = NavSignedIn::builder()
        .username(Text::from("responsiveaudit"))
        .account_href(Text::from("/protected"))
        .logout_action(Text::from("/logout"))
        .build()
        .render()
        .into_string();

    assert!(markup.contains("data-nav-auth-prefix"));
    assert!(markup.contains("data-nav-auth-name"));
    assert!(markup.contains("data-nav-account-item"));
    assert!(markup.contains("data-nav-account-link"));
    assert!(markup.contains("aria-label=\"Signed in as responsiveaudit\""));
}
