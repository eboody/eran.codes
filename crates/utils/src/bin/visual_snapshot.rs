use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use clap::{Parser, ValueEnum};
use playwright::Playwright;
use playwright::api::{ColorScheme, Viewport, page};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use url::Url;

#[derive(Debug, Parser)]
#[command(
    name = "visual_snapshot",
    about = "Capture a Playwright screenshot and optionally check against a baseline."
)]
struct Args {
    #[arg(long, default_value = "http://127.0.0.1:3000/")]
    url: Url,
    #[arg(long, default_value = "artifacts/visual/current/home.png")]
    output: PathBuf,
    #[arg(long)]
    baseline: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    update_baseline: bool,
    #[arg(long, default_value_t = 1440)]
    viewport_width: i32,
    #[arg(long, default_value_t = 1024)]
    viewport_height: i32,
    #[arg(long, default_value_t = 1200)]
    wait_ms: u64,
    #[arg(long, value_enum)]
    color_scheme: Option<SnapshotColorScheme>,
    #[arg(long)]
    element_selector: Option<String>,
    #[arg(long)]
    remove_data_init_selector: Vec<String>,
    #[arg(long)]
    click_selector: Option<String>,
    #[arg(long, default_value_t = 600)]
    click_wait_ms: u64,
    #[arg(long)]
    dump_html: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    debug_events: bool,
    #[arg(long)]
    demo_message: Option<String>,
    #[arg(long, default_value_t = 1200)]
    post_wait_ms: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)?;
    }
    if let Some(baseline) = &args.baseline
        && let Some(parent) = baseline.parent()
    {
        fs::create_dir_all(parent)?;
    }

    let playwright = Playwright::initialize().await?;
    if let Err(error) = playwright.install_chromium()
        && args.debug_events
    {
        eprintln!("[browser:install] chromium install skipped: {error}");
    }

    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    let mut events = page.subscribe_event()?;

    page.set_viewport_size(Viewport {
        width: args.viewport_width,
        height: args.viewport_height,
    })
    .await?;
    if let Some(color_scheme) = args.color_scheme {
        page.emulate_media_builder()
            .color_scheme(color_scheme.into())
            .emulate_media()
            .await?;
    }
    install_data_init_strip_script(&page, &args.remove_data_init_selector).await?;
    page.goto_builder(args.url.as_ref()).goto().await?;
    poll_page_events(&mut events, args.wait_ms, args.debug_events).await;

    if let Some(selector) = &args.click_selector {
        if args.debug_events {
            eprintln!("[browser:click] selector={selector}");
        }
        page.click_builder(selector).click().await?;
        poll_page_events(&mut events, args.click_wait_ms, args.debug_events).await;
    }

    if let Some(message) = &args.demo_message {
        let demo_input_selector =
            "[data-chat-panel-role='demo'] input[data-bind='botBody']";
        let demo_send_selector =
            "[data-chat-panel-role='demo'] button[data-chat-send='demo']";
        let demo_messages_selector =
            "[data-chat-panel-role='demo'] [data-chat-messages] > [data-chat-message]";
        if page
            .wait_for_selector_builder(demo_input_selector)
            .timeout(5_000.0)
            .wait_for_selector()
            .await?
            .is_some()
        {
            let before_messages =
                page.query_selector_all(demo_messages_selector).await?.len();
            page.fill_builder(demo_input_selector, message)
                .fill()
                .await?;
            page.click_builder(demo_send_selector).click().await?;
            poll_page_events(&mut events, args.post_wait_ms, args.debug_events).await;
            let after_messages =
                page.query_selector_all(demo_messages_selector).await?.len();
            if args.debug_events {
                eprintln!(
                    "[browser:chat] demo messages before={before_messages} after={after_messages}"
                );
            }
        } else if args.debug_events {
            eprintln!("[browser:chat] demo input not found, skipping demo post");
        }
    }

    stabilize_page_for_snapshot(&page).await?;

    if let Some(path) = &args.dump_html {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, page.content().await?)?;
    }

    let screenshot = capture_screenshot(
        &page,
        args.output.clone(),
        args.element_selector.as_deref(),
        args.debug_events,
    )
    .await?;

    browser.close().await?;

    if let Some(baseline) = args.baseline {
        if args.update_baseline {
            fs::write(&baseline, &screenshot)?;
            eprintln!("updated visual baseline: {}", baseline.display());
            return Ok(());
        }

        if !baseline.exists() {
            eprintln!(
                "baseline not found: {} (rerun with --update-baseline to create it)",
                baseline.display()
            );
            std::process::exit(2);
        }

        let baseline_bytes = fs::read(&baseline)?;
        if baseline_bytes != screenshot {
            eprintln!(
                "visual snapshot mismatch: current={} baseline={}",
                args.output.display(),
                baseline.display()
            );
            std::process::exit(2);
        }
        eprintln!("visual snapshot matches baseline: {}", baseline.display());
    } else {
        eprintln!("captured visual snapshot: {}", args.output.display());
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum SnapshotColorScheme {
    Light,
    Dark,
    NoPreference,
}

impl From<SnapshotColorScheme> for ColorScheme {
    fn from(value: SnapshotColorScheme) -> Self {
        match value {
            SnapshotColorScheme::Light => Self::Light,
            SnapshotColorScheme::Dark => Self::Dark,
            SnapshotColorScheme::NoPreference => Self::NoPreference,
        }
    }
}

async fn capture_screenshot(
    page: &playwright::api::Page,
    output: PathBuf,
    element_selector: Option<&str>,
    debug_events: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match element_selector {
        Some(selector) => {
            if debug_events {
                eprintln!("[browser:screenshot] element selector={selector}");
            }
            let Some(element) = page
                .wait_for_selector_builder(selector)
                .timeout(5_000.0)
                .wait_for_selector()
                .await?
            else {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("element selector not found: {selector}"),
                )
                .into());
            };

            Ok(element
                .screenshot_builder()
                .await
                .path(output.as_path())
                .screenshot()
                .await?)
        }
        None => Ok(page
            .screenshot_builder()
            .full_page(true)
            .path(output)
            .screenshot()
            .await?),
    }
}

async fn install_data_init_strip_script(
    page: &playwright::api::Page,
    selectors: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if selectors.is_empty() {
        return Ok(());
    }

    let selectors = serde_json::to_string(selectors)?;
    let script = format!(
        r#"
(() => {{
  const selectors = {selectors};
  const strip = () => {{
    for (const selector of selectors) {{
      for (const node of document.querySelectorAll(selector)) {{
        node.removeAttribute('data-init');
      }}
    }}
  }};

  if (document.readyState === 'loading') {{
    document.addEventListener('DOMContentLoaded', strip, {{ once: true }});
  }} else {{
    strip();
  }}
}})();
"#
    );
    page.add_init_script(&script).await?;
    Ok(())
}

async fn stabilize_page_for_snapshot(
    page: &playwright::api::Page,
) -> Result<(), Box<dyn std::error::Error>> {
    page.add_style_tag(
        r#"
*, *::before, *::after {
  animation: none !important;
  transition: none !important;
  caret-color: transparent !important;
}

html {
  scroll-behavior: auto !important;
}
"#,
        None,
    )
    .await?;
    Ok(())
}

async fn poll_page_events<S>(events: &mut S, wait_ms: u64, debug_events: bool)
where
    S: tokio_stream::Stream<Item = Result<page::Event, BroadcastStreamRecvError>> + Unpin,
{
    let deadline = Instant::now() + Duration::from_millis(wait_ms);
    while Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_millis(50), events.next()).await {
            Ok(Some(Ok(event))) => {
                if !debug_events {
                    continue;
                }
                match event {
                    page::Event::Console(message) => {
                        let level =
                            message.r#type().unwrap_or_else(|_| "console".to_owned());
                        let text = message
                            .text()
                            .unwrap_or_else(|_| "<unavailable>".to_owned());
                        eprintln!("[browser:{level}] {text}");
                    }
                    page::Event::PageError => {
                        eprintln!("[browser:error] page error emitted");
                    }
                    page::Event::Request(request) => {
                        let url =
                            request.url().unwrap_or_else(|_| "<invalid-url>".to_owned());
                        if url.contains("/events")
                            || url.contains("datastar")
                            || url.contains("/demo/chat/messages")
                        {
                            let method =
                                request.method().unwrap_or_else(|_| "GET".to_owned());
                            eprintln!("[browser:request] {method} {url}");
                        }
                    }
                    page::Event::RequestFailed(request) => {
                        let url =
                            request.url().unwrap_or_else(|_| "<invalid-url>".to_owned());
                        let reason = request
                            .failure()
                            .ok()
                            .flatten()
                            .unwrap_or_else(|| "unknown".to_owned());
                        eprintln!("[browser:request_failed] {url} ({reason})");
                    }
                    _ => {}
                }
            }
            Ok(Some(Err(error))) => {
                if debug_events {
                    eprintln!("[browser:event_error] {error}");
                }
            }
            Ok(None) => break,
            Err(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_color_scheme_maps_to_playwright_color_scheme() {
        assert_eq!(
            ColorScheme::from(SnapshotColorScheme::Light),
            ColorScheme::Light
        );
        assert_eq!(
            ColorScheme::from(SnapshotColorScheme::Dark),
            ColorScheme::Dark
        );
        assert_eq!(
            ColorScheme::from(SnapshotColorScheme::NoPreference),
            ColorScheme::NoPreference
        );
    }
}
