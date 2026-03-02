use maud::Render;
use maud_extensions::inline_js;

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Behavior;

// Tab selection here is local presentation state; Surreal wires DOM behavior directly,
// and Datastar signals are intentionally not used because no backend/shared state is involved.
inline_js! {
    (() => {
      const root = me();
      if (!root) return;

      if (root.attribute("data-showcase-bound") === "true") return;
      root.attribute("data-showcase-bound", "true");

      const tabList = me("[data-showcase-tabs][role=\"tablist\"]", root, false);
      const indicator = me("[data-showcase-tab-indicator]", root, false);
      const tabs = any("[role=\"tab\"]", root, false);
      const panels = any("[data-showcase-panel][role=\"tabpanel\"]", root, false);
      if (!tabList || !tabs.length || !panels.length) return;

      const prefersReducedMotion =
        typeof window.matchMedia === "function" &&
        window.matchMedia("(prefers-reduced-motion: reduce)").matches;

      const lastIndex = tabs.length - 1;
      let activeIndex = tabs.findIndex(
        (tab) => tab.attribute("aria-selected") === "true",
      );
      if (activeIndex < 0) activeIndex = 0;

      const updateScrollAffordance = () => {
        const hasOverflow = tabList.scrollWidth > tabList.clientWidth + 1;
        const scrolledLeft = tabList.scrollLeft > 1;
        const scrolledRight =
          tabList.scrollLeft + tabList.clientWidth < tabList.scrollWidth - 1;

        tabList.attribute({
          "data-scrollable": hasOverflow ? "true" : "false",
          "data-scroll-left": scrolledLeft ? "true" : "false",
          "data-scroll-right": scrolledRight ? "true" : "false",
        });
      };

      const moveIndicator = () => {
        updateScrollAffordance();
        if (!indicator) return;

        const activeTab = tabs[activeIndex];
        if (!activeTab) return;

        const listRect = tabList.getBoundingClientRect();
        const tabRect = activeTab.getBoundingClientRect();
        const offset = tabRect.left - listRect.left + tabList.scrollLeft;

        indicator.styles({
          transform: "translateX(" + Math.max(0, offset) + "px)",
          width: String(tabRect.width) + "px",
        });

        const tone = activeTab.attribute("data-showcase-tone");
        if (tone) tabList.attribute("data-active-tone", tone);
      };

      const syncActiveTabVisibility = (fromInteraction) => {
        if (tabList.scrollWidth <= tabList.clientWidth) {
          updateScrollAffordance();
          return;
        }
        const activeTab = tabs[activeIndex];
        if (!activeTab) return;
        const behavior =
          fromInteraction && !prefersReducedMotion ? "smooth" : "auto";
        activeTab.scrollIntoView({ inline: "center", block: "nearest", behavior });
      };

      const activate = (nextIndex, focusTab, fromInteraction) => {
        activeIndex = nextIndex;

        tabs.forEach((tab, index) => {
          const isActive = index === nextIndex;
          tab.attribute({
            "aria-selected": isActive ? "true" : "false",
            tabindex: isActive ? "0" : "-1",
          });
          if (focusTab && isActive) tab.focus();
        });

        panels.forEach((panel, index) => {
          const isActive = index === nextIndex;
          panel.hidden = !isActive;
          panel.attribute("tabindex", isActive ? "0" : "-1");
        });

        syncActiveTabVisibility(fromInteraction);
        moveIndicator();
      };

      tabs.forEach((tab, index) => {
        tab.on("click", () => activate(index, false, true));
        tab.on("keydown", (event) => {
          let next = null;
          if (event.key === "ArrowRight" || event.key === "ArrowDown") {
            next = index === lastIndex ? 0 : index + 1;
          } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
            next = index === 0 ? lastIndex : index - 1;
          } else if (event.key === "Home") {
            next = 0;
          } else if (event.key === "End") {
            next = lastIndex;
          }

          if (next !== null) {
            event.preventDefault();
            activate(next, true, true);
          }
        });
      });

      tabList.on("scroll", moveIndicator);
      window.addEventListener(
        "resize",
        () => {
          updateScrollAffordance();
          moveIndicator();
        },
        { passive: true },
      );

      if (document.fonts && document.fonts.ready) {
        document.fonts.ready
          .then(() => requestAnimationFrame(moveIndicator))
          .catch(() => {});
      }

      activate(activeIndex, false, false);
      updateScrollAffordance();
    })();
}

impl Render for Behavior {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (js())
        }
    }
}
