use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use playwright::Playwright;
use playwright::api::Viewport;
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
    playwright.install_chromium()?;

    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;

    page.set_viewport_size(Viewport {
        width: args.viewport_width,
        height: args.viewport_height,
    })
    .await?;
    page.goto_builder(args.url.as_ref()).goto().await?;
    tokio::time::sleep(Duration::from_millis(args.wait_ms)).await;

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
