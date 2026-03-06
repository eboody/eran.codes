use bon::Builder;
use maud::Render;

use crate::types::Text;

#[derive(Clone, Debug, Builder)]
pub struct NavLink {
    pub label: Text,
    pub href: Text,
    #[builder(default)]
    pub external: bool,
}

impl Render for NavLink {
    fn render(&self) -> maud::Markup {
        maud::html! {
            li {
                @if self.external {
                    a class="ui-nav-link" href=(&self.href) target="_blank" rel="noopener noreferrer" {
                        (&self.label)
                    }
                } @else {
                    a class="ui-nav-link" href=(&self.href) {
                        (&self.label)
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct NavLinkList {
    pub class_name: Text,
    pub children: Vec<NavLink>,
}

impl Render for NavLinkList {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ul class=(&self.class_name) {
                @for item in &self.children {
                    (item)
                }
            }
        }
    }
}

// ci: style-system-component
// ci: render-composition-component
#[derive(Clone, Debug, Builder)]
pub struct NavBar {
    pub brand: NavLinkList,
    pub links: NavLinkList,
    pub auth_slot: maud::Markup,
}

impl Render for NavBar {
    fn render(&self) -> maud::Markup {
        maud::html! {
            header class="container ui-nav-shell" {
                nav class="ui-nav" {
                    (&self.brand)
                    (&self.links)
                    (self.auth_slot.clone())
                }
            }
        }
    }
}
