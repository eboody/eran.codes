use maud::Markup;

const APP_CSS_ASSET_URL: &str = "/static/app.css?v=20260320-tabs";
const LOCAL_TABS_ASSET_URL: &str = "/static/local-tabs.js?v=20260325-lab-tabs";

pub(super) fn head(title: &str) -> Markup {
    maud::html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            title { (title) }
            link
                rel="icon"
                type="image/svg+xml"
                media="(prefers-color-scheme: light)"
                href="/static/eran.codes-light.svg";
            link
                rel="icon"
                type="image/svg+xml"
                media="(prefers-color-scheme: dark)"
                href="/static/eran.codes-dark.svg";
            link
                rel="icon"
                type="image/png"
                sizes="1024x1024"
                href="/static/eran.codes-favicon.png";
            link rel="apple-touch-icon" sizes="1024x1024" href="/static/eran.codes.png";
            link rel="preconnect" href="https://fonts.googleapis.com";
            link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
            link
                rel="stylesheet"
                href="https://fonts.googleapis.com/css2?family=Newsreader:opsz,wght@6..72,500;6..72,600;6..72,700&family=Space+Grotesk:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500;600&display=swap";
            link rel="stylesheet" href="/static/open-props.min.css";
            link rel="stylesheet" href=(APP_CSS_ASSET_URL);
            link
                rel="stylesheet"
                href="https://cdn.jsdelivr.net/gh/iconoir-icons/iconoir@main/css/iconoir.css";
            (crate::views::partials::components::head_styles())
            script src="/static/css-scope-inline.js" {}
            script type="module" src="/static/datastar.js" {}
            script src=(LOCAL_TABS_ASSET_URL) {}
            script type="module" src="/static/transport-errors.js" {}
        }
    }
}
