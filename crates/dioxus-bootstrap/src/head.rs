use dioxus::prelude::*;

const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");
const BOOTSTRAP_ICONS_CSS: Asset = asset!("/assets/bootstrap-icons.min.css");

/// How to load Bootstrap CSS.
///
/// Default: `Bundled` — uses the crate's embedded Bootstrap 5.3.3.
#[derive(Clone, PartialEq, Default)]
pub enum BootstrapCss {
    /// Use the crate's bundled Bootstrap 5.3.3 CSS (default, zero config).
    #[default]
    Bundled,
    /// Load from a custom URL (self-hosted, proxy to SSR backend, etc.).
    ///
    /// ```rust,ignore
    /// BootstrapHead { css: BootstrapCss::Url("/static/vendor/bootstrap.min.css".into()) }
    /// ```
    Url(String),
    /// Load from jsDelivr CDN with a specific version.
    ///
    /// ```rust,ignore
    /// BootstrapHead { css: BootstrapCss::Cdn("5.3.3".into()) }
    /// ```
    Cdn(String),
    /// Don't load any CSS — user handles it themselves.
    None,
}

/// How to load Bootstrap Icons CSS.
///
/// Default: `Bundled` — uses the crate's embedded Bootstrap Icons.
#[derive(Clone, PartialEq, Default)]
pub enum BootstrapIcons {
    /// Use the crate's bundled Bootstrap Icons CSS (default).
    #[default]
    Bundled,
    /// Load from a custom URL.
    Url(String),
    /// Don't load icons CSS — user handles it.
    None,
}

/// Loads Bootstrap CSS and Bootstrap Icons for your Dioxus app.
///
/// Place this at the top of your app, before any Bootstrap components.
///
/// # Zero config (default — bundled Bootstrap 5.3.3)
/// ```rust,ignore
/// rsx! { BootstrapHead {} }
/// ```
///
/// # Custom CSS URL (migrating from SSR with its own Bootstrap)
/// ```rust,ignore
/// rsx! {
///     BootstrapHead {
///         css: BootstrapCss::Url("/static/vendor/bootstrap.min.css".into()),
///         icons: BootstrapIcons::Url("/static/vendor/bootstrap-icons/font/bootstrap-icons.css".into()),
///     }
/// }
/// ```
///
/// # CDN with specific version
/// ```rust,ignore
/// rsx! { BootstrapHead { css: BootstrapCss::Cdn("5.3.3".into()) } }
/// ```
///
/// # Full control (load CSS yourself)
/// ```rust,ignore
/// rsx! {
///     BootstrapHead { css: BootstrapCss::None, icons: BootstrapIcons::None }
///     link { rel: "stylesheet", href: "my-custom-bootstrap.css" }
/// }
/// ```
///
/// # Embedded in a host that already loads Bootstrap
///
/// When your app renders inside a host page that already includes Bootstrap
/// (for example an outer shell), set both to `None` so it is not loaded twice:
/// ```rust,ignore
/// rsx! { BootstrapHead { css: BootstrapCss::None, icons: BootstrapIcons::None } }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct BootstrapHeadProps {
    /// How to load Bootstrap CSS. Default: bundled v5.3.3.
    #[props(default)]
    pub css: BootstrapCss,
    /// How to load Bootstrap Icons CSS. Default: bundled.
    #[props(default)]
    pub icons: BootstrapIcons,
}

#[component]
pub fn BootstrapHead(props: BootstrapHeadProps) -> Element {
    let css_element = match &props.css {
        BootstrapCss::Bundled => rsx! {
            document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        },
        BootstrapCss::Url(url) => {
            let url = url.clone();
            rsx! { link { rel: "stylesheet", href: "{url}" } }
        }
        BootstrapCss::Cdn(version) => {
            let href = format!(
                "https://cdn.jsdelivr.net/npm/bootstrap@{version}/dist/css/bootstrap.min.css"
            );
            rsx! { link { rel: "stylesheet", href: "{href}" } }
        }
        BootstrapCss::None => rsx! {},
    };

    let icons_element = match &props.icons {
        BootstrapIcons::Bundled => rsx! {
            document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS_CSS }
        },
        BootstrapIcons::Url(url) => {
            let url = url.clone();
            rsx! { link { rel: "stylesheet", href: "{url}" } }
        }
        BootstrapIcons::None => rsx! {},
    };

    rsx! {
        {css_element}
        {icons_element}
    }
}
