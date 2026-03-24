use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::views::{page, partials};

crate::views::scoped::inline_css!(
    r#"
me {
  --control-font-size: var(--text-size-body-lg);
  --control-radius: var(--radius-pill);
  --_home-hero-title-size: clamp(2.05rem, 1.45rem + 2.2vw, 3.15rem);
  --_home-hero-title-measure: 12.9ch;
  --_home-hero-title-line-height: 0.95;

  display: grid;
  gap: clamp(2.05rem, 1.35rem + 2vw, 3.2rem);
  align-items: start;
  padding: clamp(1.95rem, 1.35rem + 1.5vw, 2.95rem);
  border-radius: var(--radius-shell);
  border: 1px solid color-mix(in srgb, var(--accent-signal) 12%, var(--border-default));
  background:
    linear-gradient(
      135deg,
      color-mix(in srgb, var(--accent-signal) 11%, transparent),
      transparent 56%
    ),
    linear-gradient(
      180deg,
      color-mix(in srgb, white 34%, transparent),
      transparent 42%
    ),
    var(--surface-shell);
  box-shadow: var(--shadow-shell);
  view-transition-name: lab-hero;
}

me [data-home-hero-copy] {
  display: grid;
  gap: clamp(0.9rem, 0.7rem + 0.7vw, 1.3rem);
  align-content: start;
  min-width: 0;
  padding-inline-end: clamp(0rem, 0.3rem + 0.8vw, 1rem);
}

me [data-home-hero-kicker] {
  margin: 0;
  font-size: var(--text-size-label-sm);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-widest);
  text-transform: uppercase;
  color: var(--accent-warm);
}

me [data-home-hero-summary] {
  margin: 0;
  max-width: 58ch;
  font-size: var(--text-size-lead);
  line-height: 1.62;
  color: color-mix(in srgb, var(--text-body) 90%, var(--text-muted) 10%);
}

me [data-home-hero-title] {
  margin: 0;
  max-width: var(--_home-hero-title-measure);
  font-size: var(--_home-hero-title-size);
  line-height: var(--_home-hero-title-line-height);
  letter-spacing: var(--text-track-display);
  text-wrap: balance;
}

me [data-home-hero-tags] {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  margin-top: 0;
}

me [data-home-hero-tags] .ui-pill {
  color: var(--text-body);
  background: color-mix(in srgb, var(--surface-field) 78%, transparent);
}

me [data-home-hero-card] {
  display: grid;
  gap: var(--space-3);
  align-content: start;
  inline-size: min(100%, 23rem);
  padding: clamp(1.2rem, 0.95rem + 0.9vw, 1.75rem);
  border-radius: var(--ui-radius-md);
  border: 1px solid color-mix(in srgb, var(--accent-warm) 14%, var(--border-default));
  background:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent-warm-soft) 36%, transparent),
      transparent 56%
    ),
    var(--surface-raised);
  box-shadow: inset 0 1px 0 var(--surface-edge-default);
}

me [data-home-hero-card-title] {
  margin: 0;
  font-size: var(--text-size-label-sm);
  font-weight: 700;
  letter-spacing: var(--text-track-caps-wider);
  text-transform: uppercase;
  color: var(--text-subtle);
}

me [data-home-hero-card-status] {
  margin: 0;
  max-width: 34ch;
  font-size: var(--text-size-body-md);
  line-height: var(--text-line-summary);
  color: color-mix(in srgb, var(--text-body) 90%, var(--text-muted) 10%);
}

me [data-home-hero-card-detail] {
  margin: 0;
  max-width: 31ch;
  color: color-mix(in srgb, var(--text-muted) 94%, var(--text-body) 6%);
}

me [data-home-hero-primary-actions] [data-button-row] {
  --button-row-gap: var(--space-2);
  --button-row-item-min-inline-size: 10.5rem;
  --control-font-size: var(--text-size-body-md);
  --control-padding-block: 0.62rem;

  width: fit-content;
  margin-top: var(--space-2);
  padding-top: var(--space-4);
  border-top: 1px solid color-mix(in srgb, var(--border-subtle) 82%, transparent);
}

me [data-home-hero-card-actions] {
  display: grid;
  justify-items: start;
  margin-top: var(--space-1);
  padding-top: var(--space-3);
  gap: var(--space-2);
  border-top: 1px solid color-mix(in srgb, var(--accent-warm) 16%, var(--border-subtle));
}

me [data-home-hero-card-primary-action] {
  --control-font-size: var(--control-font-size-compact);
  --control-padding-block: 0.5rem;
  --control-padding-inline: 1rem;
  --control-radius: calc(var(--radius-control) - 2px);

  display: grid;
  min-inline-size: clamp(11.5rem, 18vw, 13.75rem);
}

me [data-home-hero-card-primary-action] :where(a.button, button, .button) {
  width: auto;
  margin: 0;
}

me [data-home-hero-card-secondary-link] {
  color: color-mix(in srgb, var(--text-muted) 82%, var(--text-body) 18%);
  font-weight: 600;
  letter-spacing: normal;
  text-decoration: none;
  text-decoration-line: underline;
  text-decoration-color: color-mix(in srgb, var(--text-muted) 42%, transparent);
  text-underline-offset: 0.18em;
}

me [data-home-hero-card-secondary-copy] {
  margin: 0;
  color: color-mix(in srgb, var(--text-muted) 92%, var(--text-body) 8%);
  font-size: var(--text-size-meta-sm);
}

me [data-home-hero-card-secondary-link]:focus-visible {
  outline: 2px solid color-mix(in srgb, var(--accent-signal) 58%, transparent);
  outline-offset: 0.2rem;
  border-radius: calc(var(--radius-control) - 3px);
}

@media (min-width: 60rem) {
  me {
    grid-template-columns: minmax(0, 1.08fr) minmax(17.5rem, 0.92fr);
  }

  me [data-home-hero-card] {
    inline-size: min(100%, 21rem);
    justify-self: end;
    margin-top: var(--space-3);
  }
}

@media (min-width: 70rem) {
  me {
    grid-template-columns: minmax(0, 1.5fr) minmax(19rem, 0.86fr);
  }

  me [data-home-hero-card] {
    justify-self: end;
    margin-top: var(--space-5);
    inline-size: min(100%, 23rem);
  }
}

@media (prefers-color-scheme: dark) {
  me {
    border-color: color-mix(in srgb, var(--accent-signal) 18%, var(--border-default));
    background:
      radial-gradient(
        circle at 16% 0%,
        color-mix(in srgb, var(--accent-signal) 18%, transparent),
        transparent 44%
      ),
      linear-gradient(180deg, var(--surface-wash-top), transparent 38%),
      color-mix(in srgb, var(--surface-shell) 94%, black 6%);
  }

  me [data-home-hero-card] {
    border-color: color-mix(in srgb, var(--accent-warm) 22%, var(--border-default));
    background:
      linear-gradient(180deg, var(--surface-wash-top-soft), transparent 36%),
      radial-gradient(
        circle at 100% 0%,
        color-mix(in srgb, var(--accent-warm) 14%, transparent),
        transparent 62%
      ),
      color-mix(in srgb, var(--surface-raised) 92%, black 8%);
    box-shadow:
      inset 0 1px 0 var(--surface-edge-default),
      0 12px 20px color-mix(in srgb, black 24%, transparent);
  }

  me [data-home-hero-card-actions] {
    border-color: color-mix(in srgb, var(--accent-warm) 24%, var(--border-default));
  }
}

@media (max-width: 48rem) {
  me {
    --_home-hero-title-size: clamp(1.9rem, 1.5rem + 2.2vw, 2.6rem);
    --_home-hero-title-measure: 15.2ch;
    padding: clamp(1.2rem, 1rem + 1vw, 1.65rem);
    gap: var(--space-4);
  }

  me [data-home-hero-copy] {
    gap: var(--space-4);
    padding-inline-end: 0;
  }

  me [data-home-hero-summary] {
    font-size: var(--text-size-body-lg);
  }

  me [data-home-hero-tags] {
    gap: var(--space-1);
  }

  me [data-home-hero-tags] > :nth-child(n + 3) {
    display: none;
  }

  me [data-home-hero-card] {
    inline-size: 100%;
    gap: var(--space-2);
    padding: var(--space-4);
  }

  me [data-home-hero-card-status] {
    font-size: var(--text-size-body-md);
  }

  me [data-home-hero-card-detail] {
    font-size: var(--text-size-meta-sm);
  }

  me [data-home-hero-card-actions] {
    gap: var(--space-1);
    padding-top: var(--space-2);
    justify-items: stretch;
  }

  me [data-home-hero-card-primary-action] {
    min-inline-size: 0;
  }

  me [data-home-hero-primary-actions] [data-button-row] {
    width: 100%;
    margin-top: var(--space-1);
    padding-top: var(--space-2);
  }
}

@media (max-width: 36rem) {
  me {
    --_home-hero-title-measure: 14.1ch;
  }
}

@media (hover: hover) {
  me [data-home-hero-card-secondary-link]:hover {
    color: var(--text-strong);
    text-decoration-color: currentColor;
  }
}

@media (max-width: 24rem) {
  me {
    --_home-hero-title-measure: 11.4ch;
  }
}
"#
);

#[derive(Clone, Debug, Builder)]
pub struct HomeHero {
    pub user: Option<page::UserNav>,
}

impl Render for HomeHero {
    fn render(&self) -> maud::Markup {
        let content = partials::components::portfolio::content::lab_page_content();

        maud::html! {
            header id="home-hero" data-home-hero {
                (css())
                div data-home-hero-copy {
                    p data-home-hero-kicker { (&content.hero.eyebrow) }
                    h1 data-home-hero-title { (&content.hero.title) }
                    p data-home-hero-summary { (&content.hero.summary) }
                    div data-home-hero-tags {
                        @for badge in &content.hero.badges {
                            (partials::components::Pill::builder().text(badge.clone()).build())
                        }
                    }
                    div data-home-hero-primary-actions {
                        (partials::button::Row::builder()
                            .density(partials::button::RowDensity::Compact)
                            .narrow_layout(partials::button::RowNarrowLayout::Stack)
                            .items(content.hero.actions.iter().map(|action| {
                                partials::button::Button::builder()
                                    .label(action.label.clone())
                                    .variant(match action.tone {
                                        partials::components::portfolio::content::CtaKind::Primary => {
                                            partials::button::Variant::Primary
                                        }
                                        partials::components::portfolio::content::CtaKind::Secondary => {
                                            partials::button::Variant::Secondary
                                        }
                                    })
                                    .role(if action.kind.is_external() {
                                        partials::button::Role::external_link(action.href.clone())
                                    } else {
                                        partials::button::Role::link(action.href.clone())
                                    })
                                    .build()
                            }).collect())
                            .build())
                    }
                }
                aside data-home-hero-card {
                    h3 data-home-hero-card-title { (&content.session_card.title) }
                    @if let Some(user) = &self.user {
                        p data-home-hero-card-status { "Signed in as " strong { (&user.username) } "." }
                        p class="u-muted" data-home-hero-card-detail { (&user.email) }
                        div data-home-hero-card-actions {
                            (partials::button::Row::builder()
                                .density(partials::button::RowDensity::Compact)
                                .narrow_layout(partials::button::RowNarrowLayout::Stack)
                                .items(vec![
                                    partials::button::Button::builder()
                                        .label(content.session_card.signed_in_action_label.clone())
                                        .variant(partials::button::Variant::Primary)
                                        .role(partials::button::Role::link(Route::Protected.as_str()))
                                        .build(),
                                ])
                                .build())
                        }
                    } @else {
                        p data-home-hero-card-status { (&content.session_card.guest_status) }
                        p class="u-muted" data-home-hero-card-detail { (&content.session_card.guest_summary) }
                        div data-home-hero-card-actions {
                            @if let Some(primary_action) = content.session_card.guest_actions.first() {
                                div data-home-hero-card-primary-action {
                                    (partials::button::Button::builder()
                                        .label(primary_action.label.clone())
                                        .variant(partials::button::Variant::Primary)
                                        .role(partials::button::Role::link(primary_action.href.clone()))
                                        .build())
                                }
                            }
                            @if let Some(secondary_action) = content.session_card.guest_actions.get(1) {
                                p data-home-hero-card-secondary-copy {
                                    "Already have an account? "
                                    a
                                        href=(secondary_action.href.clone())
                                        data-home-hero-card-secondary-link
                                    {
                                        (&secondary_action.label)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use super::*;

    #[test]
    fn renders_explicit_hero_hooks() {
        let markup = HomeHero::builder().build().render().into_string();

        assert!(markup.contains("data-home-hero-title"));
        assert!(markup.contains("data-home-hero-summary"));
        assert!(markup.contains("data-home-hero-primary-actions"));
        assert!(markup.contains("data-home-hero-card-actions"));
        assert!(markup.contains("data-home-hero-card-secondary-link"));
        assert!(markup.contains("data-home-hero-card-secondary-copy"));
    }
}
