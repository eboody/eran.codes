use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use clap::Parser;
use playwright::Playwright;
use playwright::api::Viewport;
use playwright::api::page::Event as PageEvent;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use url::Url;

#[derive(Debug, Parser)]
#[command(
    name = "visual_snapshot",
    about = "Capture a full-page Playwright screenshot and optionally check against a baseline."
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
    page.goto_builder(args.url.as_ref()).goto().await?;
    poll_page_events(&mut events, args.wait_ms, args.debug_events).await;

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

    if let Some(path) = &args.dump_html {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, page.content().await?)?;
    }

    let screenshot = page
        .screenshot_builder()
        .full_page(true)
        .path(args.output.clone())
        .screenshot()
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

async fn poll_page_events<S>(events: &mut S, wait_ms: u64, debug_events: bool)
where
    S: tokio_stream::Stream<Item = Result<PageEvent, BroadcastStreamRecvError>> + Unpin,
{
    let deadline = Instant::now() + Duration::from_millis(wait_ms);
    while Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_millis(50), events.next()).await {
            Ok(Some(Ok(event))) => {
                if !debug_events {
                    continue;
                }
                match event {
                    PageEvent::Console(message) => {
                        let level =
                            message.r#type().unwrap_or_else(|_| "console".to_owned());
                        let text = message
                            .text()
                            .unwrap_or_else(|_| "<unavailable>".to_owned());
                        eprintln!("[browser:{level}] {text}");
                    }
                    PageEvent::PageError => {
                        eprintln!("[browser:error] page error emitted");
                    }
                    PageEvent::Request(request) => {
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
                    PageEvent::RequestFailed(request) => {
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
