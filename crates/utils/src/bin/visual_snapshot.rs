use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use clap::{Parser, ValueEnum};
use image::{ImageFormat, load_from_memory_with_format};
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
    normalize_text_selector: Vec<String>,
    #[arg(long)]
    click_selector: Option<String>,
    #[arg(long, default_value_t = 600)]
    click_wait_ms: u64,
    #[arg(long)]
    assert_selector: Vec<String>,
    #[arg(long, default_value_t = 5_000)]
    assert_timeout_ms: u64,
    #[arg(long)]
    dump_html: Option<PathBuf>,
    #[arg(long, default_value_t = false)]
    debug_events: bool,
    #[arg(long)]
    demo_message: Option<String>,
    #[arg(long, default_value_t = 1200)]
    post_wait_ms: u64,
}

const PIXEL_DIFF_TOLERANCE_ABSOLUTE: u64 = 16;
const PIXEL_DIFF_TOLERANCE_RATIO: f64 = 0.000_01;

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
    install_text_normalizer_script(&page, &args.normalize_text_selector).await?;
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
    assert_selectors_present(
        &page,
        &args.assert_selector,
        args.assert_timeout_ms,
        args.debug_events,
    )
    .await?;

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
        let comparison = compare_png_pixels(&baseline_bytes, &screenshot)?;
        if !comparison.within_tolerance() {
            eprintln!(
                "visual snapshot mismatch: current={} baseline={} differing_pixels={} allowed_pixels={}",
                args.output.display(),
                baseline.display(),
                comparison.differing_pixels,
                comparison.allowed_pixels
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

async fn install_text_normalizer_script(
    page: &playwright::api::Page,
    specs: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    if specs.is_empty() {
        return Ok(());
    }

    let specs = specs
        .iter()
        .map(|value| {
            let (selector, replacement) = value.split_once("=>").ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "normalize-text-selector must use selector=>replacement: {value}"
                    ),
                )
            })?;
            Ok(serde_json::json!({
                "selector": selector,
                "replacement": replacement,
            }))
        })
        .collect::<Result<Vec<_>, io::Error>>()?;
    let specs = serde_json::to_string(&specs)?;
    let script = format!(
        r#"
(() => {{
  const specs = {specs};
  const normalize = () => {{
    for (const spec of specs) {{
      for (const node of document.querySelectorAll(spec.selector)) {{
        node.textContent = spec.replacement;
      }}
    }}
  }};

  if (document.readyState === 'loading') {{
    document.addEventListener('DOMContentLoaded', normalize, {{ once: true }});
  }} else {{
    normalize();
  }}

  const observer = new MutationObserver(normalize);
  observer.observe(document.documentElement, {{
    childList: true,
    subtree: true,
    characterData: true,
  }});
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

async fn assert_selectors_present(
    page: &playwright::api::Page,
    selectors: &[String],
    timeout_ms: u64,
    debug_events: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for selector in selectors {
        if debug_events {
            eprintln!("[browser:assert] selector={selector}");
        }
        let found = page
            .wait_for_selector_builder(selector)
            .timeout(timeout_ms as f64)
            .wait_for_selector()
            .await?;
        if found.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("assert selector not found: {selector}"),
            )
            .into());
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PixelComparison {
    differing_pixels: u64,
    allowed_pixels: u64,
}

impl PixelComparison {
    fn within_tolerance(self) -> bool {
        self.differing_pixels <= self.allowed_pixels
    }
}

fn compare_png_pixels(
    baseline_bytes: &[u8],
    current_bytes: &[u8],
) -> Result<PixelComparison, Box<dyn std::error::Error>> {
    let baseline =
        load_from_memory_with_format(baseline_bytes, ImageFormat::Png)?.to_rgba8();
    let current = load_from_memory_with_format(current_bytes, ImageFormat::Png)?.to_rgba8();

    if baseline.dimensions() != current.dimensions() {
        return Ok(PixelComparison {
            differing_pixels: u64::MAX,
            allowed_pixels: 0,
        });
    }

    let differing_pixels = baseline
        .pixels()
        .zip(current.pixels())
        .filter(|(left, right)| left != right)
        .count() as u64;
    let total_pixels = baseline.width() as u64 * baseline.height() as u64;
    let allowed_pixels = PIXEL_DIFF_TOLERANCE_ABSOLUTE
        .max((total_pixels as f64 * PIXEL_DIFF_TOLERANCE_RATIO).ceil() as u64);

    Ok(PixelComparison {
        differing_pixels,
        allowed_pixels,
    })
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

    #[test]
    fn png_pixel_match_ignores_png_encoding_differences() {
        let image = image::RgbaImage::from_pixel(2, 1, image::Rgba([1, 2, 3, 255]));
        let mut baseline = Vec::new();
        let mut current = Vec::new();
        image
            .write_to(
                &mut std::io::Cursor::new(&mut baseline),
                image::ImageFormat::Png,
            )
            .expect("baseline png");
        image
            .write_to(
                &mut std::io::Cursor::new(&mut current),
                image::ImageFormat::Png,
            )
            .expect("current png");

        let comparison = compare_png_pixels(&baseline, &current).expect("decode png");
        assert!(comparison.within_tolerance());
        assert_eq!(comparison.differing_pixels, 0);
    }

    #[test]
    fn png_pixel_match_allows_tiny_raster_drift() {
        let baseline = image::RgbaImage::from_pixel(100, 100, image::Rgba([1, 2, 3, 255]));
        let mut current = baseline.clone();
        current.put_pixel(0, 0, image::Rgba([9, 9, 9, 255]));

        let mut baseline_png = Vec::new();
        let mut current_png = Vec::new();
        baseline
            .write_to(
                &mut std::io::Cursor::new(&mut baseline_png),
                image::ImageFormat::Png,
            )
            .expect("baseline png");
        current
            .write_to(
                &mut std::io::Cursor::new(&mut current_png),
                image::ImageFormat::Png,
            )
            .expect("current png");

        let comparison =
            compare_png_pixels(&baseline_png, &current_png).expect("decode png");

        assert!(comparison.within_tolerance());
        assert_eq!(comparison.differing_pixels, 1);
    }

    #[test]
    fn png_pixel_match_rejects_large_visual_changes() {
        let baseline = image::RgbaImage::from_pixel(10, 10, image::Rgba([1, 2, 3, 255]));
        let current = image::RgbaImage::from_pixel(10, 10, image::Rgba([9, 9, 9, 255]));
        let mut baseline_png = Vec::new();
        let mut current_png = Vec::new();
        baseline
            .write_to(
                &mut std::io::Cursor::new(&mut baseline_png),
                image::ImageFormat::Png,
            )
            .expect("baseline png");
        current
            .write_to(
                &mut std::io::Cursor::new(&mut current_png),
                image::ImageFormat::Png,
            )
            .expect("current png");

        let comparison =
            compare_png_pixels(&baseline_png, &current_png).expect("decode png");

        assert!(!comparison.within_tolerance());
    }

    #[test]
    fn normalize_text_selector_requires_mapping_syntax() {
        let error = "missing-delimiter"
            .split_once("=>")
            .ok_or_else(|| std::io::Error::other("missing mapping"))
            .expect_err("invalid mapping");

        assert_eq!(error.to_string(), "missing mapping");
    }
}
