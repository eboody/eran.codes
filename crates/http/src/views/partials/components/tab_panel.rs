use maud::Render;
use maud_extensions::{inline_css, inline_js};

use crate::types::Text;

use super::Tab;

#[derive(Clone, Debug)]
pub(crate) struct TabPanel<'a> {
    pub tabs: &'a [Tab],
    pub aria_label: Text,
}

impl Render for TabPanel<'_> {
    fn render(&self) -> maud::Markup {
        let tabs = Self::normalize_selected_tabs(self.tabs);

        maud::html! {
            nav class="showcase-tabs" role="tablist" aria-label=(&self.aria_label) {
                (css())
                @for tab in &tabs {
                    (tab)
                }
            }
            (js())
        }
    }
}

impl TabPanel<'_> {
    pub(crate) fn normalize_selected_tabs(tabs: &[Tab]) -> Vec<Tab> {
        let selected_index = tabs.iter().position(|tab| tab.is_selected).unwrap_or(0);

        tabs
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, mut tab)| {
                tab.is_selected = index == selected_index;
                tab
            })
            .collect()
    }
}

inline_css! {
    me {
      display: flex;
      align-items: stretch;
      gap: var(--size-2);
      overflow-x: auto;
      padding-bottom: var(--size-1);
      border-bottom: var(--border-size-1) solid var(--surface-shell-border);
    }
}

inline_js! {
    (() => {
      const TABLIST_SELECTOR = "nav.showcase-tabs[role='tablist']";
      const TAB_SELECTOR = "button[role='tab']";
      const READY_ATTR = "data-tablist-ready";

      const getTabs = (tablist) => Array.from(tablist.querySelectorAll(TAB_SELECTOR));

      const syncPanels = (tablist, activeTab, tabs) => {
        tabs.forEach((tab) => {
          const panelId = tab.getAttribute("aria-controls");
          if (!panelId) return;

          const panel = document.getElementById(panelId);
          if (!panel) return;

          const isActive = tab === activeTab;
          panel.hidden = !isActive;
          panel.tabIndex = isActive ? 0 : -1;
        });
      };

      const activateTab = (tablist, activeTab, { focus = true } = {}) => {
        const tabs = getTabs(tablist);
        if (!tabs.length) return;

        tabs.forEach((tab) => {
          const isActive = tab === activeTab;
          tab.setAttribute("aria-selected", isActive ? "true" : "false");
          tab.setAttribute("tabindex", isActive ? "0" : "-1");
          tab.classList.toggle("is-selected", isActive);
        });

        syncPanels(tablist, activeTab, tabs);

        if (focus) {
          activeTab.focus();
        }
      };

      const wireTablist = (tablist) => {
        if (tablist.hasAttribute(READY_ATTR)) return;
        tablist.setAttribute(READY_ATTR, "true");

        const initialTabs = getTabs(tablist);
        if (!initialTabs.length) return;

        const initiallySelected =
          initialTabs.find((tab) => tab.getAttribute("aria-selected") === "true") ||
          initialTabs[0];

        activateTab(tablist, initiallySelected, { focus: false });

        tablist.addEventListener("click", (event) => {
          const tab = event.target.closest(TAB_SELECTOR);
          if (!tab || !tablist.contains(tab)) return;

          event.preventDefault();
          activateTab(tablist, tab, { focus: true });
        });

        tablist.addEventListener("keydown", (event) => {
          const tab = event.target.closest(TAB_SELECTOR);
          if (!tab || !tablist.contains(tab)) return;

          const tabs = getTabs(tablist);
          const currentIndex = tabs.indexOf(tab);
          if (currentIndex < 0) return;

          let nextIndex = null;
          switch (event.key) {
            case "ArrowRight":
            case "ArrowDown":
              nextIndex = (currentIndex + 1) % tabs.length;
              break;
            case "ArrowLeft":
            case "ArrowUp":
              nextIndex = (currentIndex - 1 + tabs.length) % tabs.length;
              break;
            case "Home":
              nextIndex = 0;
              break;
            case "End":
              nextIndex = tabs.length - 1;
              break;
            case "Enter":
            case " ":
              nextIndex = currentIndex;
              break;
            default:
              break;
          }

          if (nextIndex === null) return;
          event.preventDefault();
          activateTab(tablist, tabs[nextIndex], { focus: true });
        });
      };

      document.querySelectorAll(TABLIST_SELECTOR).forEach(wireTablist);
    })();
}
