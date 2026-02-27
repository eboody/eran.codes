use maud::Render;
use maud_extensions::css;

#[derive(Clone, Copy, Debug, Default)]
pub struct Styles;

impl Render for Styles {
    fn render(&self) -> maud::Markup {
        maud::html! {
            ({
                css! {
                    me [data-log-panels] {
                      display: grid;
                      gap: 1.5rem;
                      grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
                    }
                    me [data-log-panel] {
                      display: flex;
                      flex-direction: column;
                      gap: 0.75rem;
                    }
                    me [data-demo-result] {
                      margin-top: 0.8rem;
                      padding: 0.8rem 1rem;
                      border-radius: var(--ui-radius-sm);
                      border: 1px solid var(--pico-muted-border-color);
                      background: var(--pico-card-background-color);
                    }
                    me [data-log-heading] h3 {
                      margin: 0;
                      font-size: 0.95rem;
                    }
                    me [data-log-scroll] {
                      max-height: 320px;
                      overflow: auto;
                      display: flex;
                      flex-direction: column;
                      gap: 1rem;
                      min-height: 0;
                      padding-right: 0.2rem;
                      scrollbar-gutter: stable both-edges;
                      box-shadow:
                        inset 12px 0 12px -12px rgba(0, 0, 0, 0.35),
                        inset -12px 0 12px -12px rgba(0, 0, 0, 0.35);
                    }
                    me [data-live-log] {
                      display: flex;
                      flex-direction: column;
                    }
                    me [data-live-log-entries] {
                      list-style: none;
                      margin: 0;
                      padding: 0;
                      display: flex;
                      flex-direction: column;
                      gap: 0.75rem;
                    }
                    me [data-log-entry] {
                      display: flex;
                      flex-wrap: wrap;
                      align-items: center;
                      gap: 0.5rem;
                      width: 100%;
                    }
                    me [data-log-timestamp] {
                      font-variant-numeric: tabular-nums;
                      font-size: 0.72rem;
                      line-height: 1;
                    }
                    me [data-log-message] {
                      font-weight: 600;
                      white-space: normal;
                    }
                    me .log-fields {
                      font-size: 0.7rem;
                      letter-spacing: 0.01rem;
                      display: inline-flex;
                      flex-wrap: wrap;
                      gap: 0.4rem;
                    }
                    me table[data-log-table] {
                      width: 100%;
                      border-collapse: collapse;
                      font-size: 0.8rem;
                    }
                    me table[data-log-table] th,
                    me table[data-log-table] td {
                      padding: 0.45rem 0.4rem;
                      border-bottom: 1px solid
                        color-mix(in srgb, var(--pico-muted-color) 28%, transparent);
                      vertical-align: top;
                    }
                    me table[data-log-table] th {
                      text-align: left;
                      color: var(--pico-muted-color);
                      font-weight: 600;
                      white-space: nowrap;
                    }
                    me table[data-log-table][data-chat-flow] td:last-child {
                      min-width: 140px;
                    }
                    me [data-log-groups] {
                      display: flex;
                      flex-direction: column;
                      gap: 0.6rem;
                    }
                    me [data-log-group] {
                      border: 1px solid color-mix(in srgb, var(--pico-muted-color) 24%, transparent);
                      border-radius: var(--ui-radius-sm);
                      padding: 0.5rem 0.6rem;
                      background: color-mix(
                        in srgb,
                        var(--pico-card-background-color) 88%,
                        var(--pico-muted-color) 12%
                      );
                    }
                    me [data-log-group-header] {
                      display: flex;
                      align-items: center;
                      gap: 0.5rem;
                      margin-bottom: 0.4rem;
                    }
                    @media (max-width: 768px) {
                      me [data-log-panels] {
                        grid-template-columns: 1fr;
                        gap: 1rem;
                      }
                      me [data-log-scroll] {
                        max-height: 260px;
                        box-shadow: none;
                        padding-right: 0;
                      }
                      me table[data-log-table] {
                        min-width: 560px;
                      }
                      me [data-log-group-header] {
                        flex-wrap: wrap;
                      }
                    }
                    @media (max-width: 480px) {
                      me [data-log-scroll] {
                        max-height: 220px;
                      }
                      me table[data-log-table] {
                        min-width: 520px;
                        font-size: 0.74rem;
                      }
                      me [data-log-entry] {
                        align-items: flex-start;
                      }
                    }
                }
            })
        }
    }
}
